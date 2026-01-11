import React, { createContext, useContext, useState, useEffect, type ReactNode } from 'react';

// Types for the Casper Wallet extension injection
interface CasperWalletProvider {
    requestConnection: () => Promise<boolean>;
    isConnected: () => Promise<boolean>;
    getActivePublicKey: () => Promise<string>;
    sign: (deployJson: string, publicKey: string) => Promise<string>; // Returns content of signed deploy
    disconnectFromSite: () => Promise<boolean>;
}

// Extend window interface
declare global {
    interface Window {
        CasperWalletProvider?: () => CasperWalletProvider;
    }
}

interface WalletContextType {
    activeKey: string | null;
    isConnected: boolean;
    isConnecting: boolean;
    balance: number | null; // Balance in CSPR
    connect: () => Promise<void>;
    disconnect: () => Promise<void>;
    signDeploy: (deploy: any) => Promise<any | null>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

// Testnet RPC Node
const NODE_URL = import.meta.env.VITE_CASPER_NODE_URL || 'http://136.243.187.84:7777/rpc';

export const WalletProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [activeKey, setActiveKey] = useState<string | null>(null);
    const [isConnected, setIsConnected] = useState(false);
    const [isConnecting, setIsConnecting] = useState(false);
    const [balance, setBalance] = useState<number | null>(null);

    // Poll for wallet availability and initial state
    useEffect(() => {
        const checkConnection = async () => {
            const provider = window.CasperWalletProvider?.();
            if (provider) {
                try {
                    const connected = await provider.isConnected();
                    if (connected) {
                        const key = await provider.getActivePublicKey();
                        setActiveKey(key);
                        setIsConnected(true);
                    }
                } catch (err) {
                    console.error("Failed to check wallet connection:", err);
                }
            }
        };

        checkConnection();

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const handleEvent = (event: any) => {
            try {
                const state = JSON.parse(event.detail);
                if (state.activeKey) {
                    setActiveKey(state.activeKey);
                    setIsConnected(true);
                } else {
                    setActiveKey(null);
                    setIsConnected(false);
                    setBalance(null);
                }
            } catch (e) {
                console.error(e);
            }
        };

        window.addEventListener('casper-wallet:activeKeyChanged', handleEvent);
        window.addEventListener('casper-wallet:connected', handleEvent);
        window.addEventListener('casper-wallet:disconnected', handleEvent);

        return () => {
            window.removeEventListener('casper-wallet:activeKeyChanged', handleEvent);
            window.removeEventListener('casper-wallet:connected', handleEvent);
            window.removeEventListener('casper-wallet:disconnected', handleEvent);
        };
    }, []);

    // Fetch balance when activeKey changes
    useEffect(() => {
        const getBalance = async () => {
            if (!activeKey || !isConnected) {
                setBalance(null);
                return;
            }

            try {
                // Helper for JSON-RPC calls
                const rpcCall = async (method: string, params: any) => {
                    const response = await fetch(NODE_URL, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            id: 1,
                            jsonrpc: '2.0',
                            method,
                            params
                        })
                    });
                    const data = await response.json();
                    if (data.error) throw new Error(data.error.message);
                    return data.result;
                };

                // 1. Get latest block for state root hash
                const chainInfo = await rpcCall('chain_get_block', {});
                const stateRootHash = chainInfo.block.header.state_root_hash;

                // 2. Get Account Info to find Main Purse
                const stateItem = await rpcCall('state_get_item', {
                    state_root_hash: stateRootHash,
                    key: activeKey,
                    path: []
                });

                const mainPurse = stateItem.stored_value.Account.main_purse;

                // 3. Get Balance of Main Purse
                const balanceInfo = await rpcCall('state_get_balance', {
                    state_root_hash: stateRootHash,
                    purse_uref: mainPurse
                });

                const balanceMotes = balanceInfo.balance_value;
                // Convert motes to CSPR (1 CSPR = 1,000,000,000 motes)
                const balanceCspr = parseInt(balanceMotes) / 1_000_000_000;

                setBalance(balanceCspr);

            } catch (err) {
                console.error("Failed to fetch balance:", err);
            }
        };

        getBalance();
    }, [activeKey, isConnected]);

    const connect = async () => {
        setIsConnecting(true);
        try {
            const provider = window.CasperWalletProvider?.();
            if (!provider) {
                alert("Casper Wallet not detected. Please install it!");
                window.open('https://www.casperwallet.io/', '_blank');
                return;
            }

            const success = await provider.requestConnection();
            if (success) {
                const key = await provider.getActivePublicKey();
                setActiveKey(key);
                setIsConnected(true);
            }
        } catch (err) {
            console.error("Connection failed:", err);
        } finally {
            setIsConnecting(false);
        }
    };

    const disconnect = async () => {
        try {
            const provider = window.CasperWalletProvider?.();
            if (provider) {
                await provider.disconnectFromSite();
            }
        } catch (err) {
            console.error("Disconnect error", err);
        }
        setActiveKey(null);
        setIsConnected(false);
        setBalance(null);
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const signDeploy = async (deploy: any): Promise<any | null> => {
        const provider = window.CasperWalletProvider?.();
        if (!provider || !activeKey) throw new Error("Wallet not connected");

        const deployJsonString = typeof deploy === 'string' ? deploy : JSON.stringify(deploy);

        try {
            const signedDeployJson = await provider.sign(deployJsonString, activeKey);
            return JSON.parse(signedDeployJson);
        } catch (err) {
            console.error("Signing failed:", err);
            return null;
        }
    };

    return (
        <WalletContext.Provider value={{ activeKey, isConnected, isConnecting, balance, connect, disconnect, signDeploy }}>
            {children}
        </WalletContext.Provider>
    );
};

export const useWallet = () => {
    const context = useContext(WalletContext);
    if (context === undefined) {
        throw new Error('useWallet must be used within a WalletProvider');
    }
    return context;
};
