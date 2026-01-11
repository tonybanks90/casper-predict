import React, { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import { useWallet } from './WalletContext';

// ============================================================================
// Types
// ============================================================================

export interface Market {
    id: number;
    question: string;
    category: string;
    marketType: 'binary' | 'multiple_choice';
    outcomes: string[];
    endTime: Date;
    creator: string;
    createdAt: Date;
    totalVolume: number; // in CSPR
    yesPercentage: number;
    noPercentage: number;
    status: 'active' | 'closed' | 'resolved';
}

export interface UserPosition {
    marketId: number;
    outcomeId: number;
    shares: number;
    avgCost: number;
}

interface MarketContextType {
    markets: Market[];
    userPositions: UserPosition[];
    isLoading: boolean;
    createMarket: (params: CreateMarketParams) => Promise<number | null>;
    buyShares: (marketId: number, outcomeId: number, amount: number) => Promise<boolean>;
    sellShares: (marketId: number, outcomeId: number, shares: number) => Promise<boolean>;
    fetchMarkets: () => Promise<void>;
    getMarket: (id: number) => Market | undefined;
}

interface CreateMarketParams {
    question: string;
    category: string;
    endTime: Date;
    initialLiquidity: number; // in CSPR
    marketType: 'binary' | 'multiple_choice';
    outcomes?: string[]; // for multiple choice
}

const MarketContext = createContext<MarketContextType | undefined>(undefined);

// ============================================================================
// Mock Data for Local Development
// ============================================================================

const generateMockMarkets = (): Market[] => [
    {
        id: 0,
        question: "Will BTC reach $100k by end of 2026?",
        category: "crypto",
        marketType: 'binary',
        outcomes: ['Yes', 'No'],
        endTime: new Date('2026-12-31'),
        creator: '01abc...def',
        createdAt: new Date('2026-01-01'),
        totalVolume: 12450,
        yesPercentage: 67,
        noPercentage: 33,
        status: 'active',
    },
    {
        id: 1,
        question: "Will Lakers win the 2026 NBA Finals?",
        category: "sports",
        marketType: 'binary',
        outcomes: ['Yes', 'No'],
        endTime: new Date('2026-06-15'),
        creator: '01xyz...789',
        createdAt: new Date('2026-01-05'),
        totalVolume: 8320,
        yesPercentage: 45,
        noPercentage: 55,
        status: 'active',
    },
    {
        id: 2,
        question: "Who will win the 2026 World Cup?",
        category: "sports",
        marketType: 'multiple_choice',
        outcomes: ['Brazil', 'Argentina', 'France', 'Germany'],
        endTime: new Date('2026-07-19'),
        creator: '01def...456',
        createdAt: new Date('2026-01-08'),
        totalVolume: 25000,
        yesPercentage: 30,
        noPercentage: 70,
        status: 'active',
    },
    {
        id: 3,
        question: "Will ETH flip BTC in market cap?",
        category: "crypto",
        marketType: 'binary',
        outcomes: ['Yes', 'No'],
        endTime: new Date('2026-06-30'),
        creator: '01ghi...012',
        createdAt: new Date('2026-01-10'),
        totalVolume: 5670,
        yesPercentage: 22,
        noPercentage: 78,
        status: 'active',
    },
    {
        id: 4,
        question: "Will it rain in London on July 4th?",
        category: "weather",
        marketType: 'binary',
        outcomes: ['Yes', 'No'],
        endTime: new Date('2026-07-04'),
        creator: '01jkl...345',
        createdAt: new Date('2026-01-11'),
        totalVolume: 1200,
        yesPercentage: 85,
        noPercentage: 15,
        status: 'active',
    },
    {
        id: 5,
        question: "Best programming language 2026?",
        category: "tech",
        marketType: 'multiple_choice',
        outcomes: ['Rust', 'Go', 'Python', 'TypeScript'],
        endTime: new Date('2026-12-01'),
        creator: '01mno...678',
        createdAt: new Date('2026-01-12'),
        totalVolume: 3400,
        yesPercentage: 40,
        noPercentage: 60,
        status: 'active',
    },
];

// ============================================================================
// Provider
// ============================================================================

export const MarketProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const { isConnected, activeKey } = useWallet();
    const [markets, setMarkets] = useState<Market[]>(generateMockMarkets());
    const [userPositions, setUserPositions] = useState<UserPosition[]>([]);
    const [isLoading, setIsLoading] = useState(false);

    const fetchMarkets = useCallback(async () => {
        setIsLoading(true);
        try {
            // TODO: Replace with actual RPC call when testnet is available
            await new Promise(resolve => setTimeout(resolve, 500));
            setMarkets(generateMockMarkets());
        } catch (error) {
            console.error('Failed to fetch markets:', error);
        } finally {
            setIsLoading(false);
        }
    }, []);

    const createMarket = useCallback(async (params: CreateMarketParams): Promise<number | null> => {
        if (!isConnected || !activeKey) {
            console.error('Wallet not connected');
            return null;
        }

        setIsLoading(true);
        try {
            // TODO: Replace with actual contract call
            await new Promise(resolve => setTimeout(resolve, 1000));

            const newMarket: Market = {
                id: markets.length,
                question: params.question,
                category: params.category,
                marketType: params.marketType,
                outcomes: params.marketType === 'binary' ? ['Yes', 'No'] : (params.outcomes || []),
                endTime: params.endTime,
                creator: activeKey.slice(0, 10) + '...',
                createdAt: new Date(),
                totalVolume: params.initialLiquidity,
                yesPercentage: 50,
                noPercentage: 50,
                status: 'active',
            };

            setMarkets(prev => [...prev, newMarket]);
            console.log('[MarketContext] Created market:', newMarket.id);
            return newMarket.id;
        } catch (error) {
            console.error('Failed to create market:', error);
            return null;
        } finally {
            setIsLoading(false);
        }
    }, [isConnected, activeKey, markets.length]);

    const buyShares = useCallback(async (
        marketId: number,
        outcomeId: number,
        amount: number
    ): Promise<boolean> => {
        if (!isConnected || !activeKey) {
            console.error('Wallet not connected');
            return false;
        }

        setIsLoading(true);
        try {
            // TODO: Replace with actual contract call
            await new Promise(resolve => setTimeout(resolve, 1000));

            setMarkets(prev => prev.map(m =>
                m.id === marketId
                    ? { ...m, totalVolume: m.totalVolume + amount }
                    : m
            ));

            const existingPos = userPositions.find(
                p => p.marketId === marketId && p.outcomeId === outcomeId
            );

            if (existingPos) {
                setUserPositions(prev => prev.map(p =>
                    p.marketId === marketId && p.outcomeId === outcomeId
                        ? { ...p, shares: p.shares + amount, avgCost: (p.avgCost + amount) / 2 }
                        : p
                ));
            } else {
                setUserPositions(prev => [...prev, {
                    marketId,
                    outcomeId,
                    shares: amount,
                    avgCost: amount,
                }]);
            }

            console.log(`[MarketContext] Bought ${amount} shares on market ${marketId}, outcome ${outcomeId}`);
            return true;
        } catch (error) {
            console.error('Failed to buy shares:', error);
            return false;
        } finally {
            setIsLoading(false);
        }
    }, [isConnected, activeKey, userPositions]);

    const sellShares = useCallback(async (
        marketId: number,
        outcomeId: number,
        shares: number
    ): Promise<boolean> => {
        if (!isConnected) {
            console.error('Wallet not connected');
            return false;
        }

        setIsLoading(true);
        try {
            await new Promise(resolve => setTimeout(resolve, 1000));

            setUserPositions(prev => prev.map(p =>
                p.marketId === marketId && p.outcomeId === outcomeId
                    ? { ...p, shares: Math.max(0, p.shares - shares) }
                    : p
            ).filter(p => p.shares > 0));

            console.log(`[MarketContext] Sold ${shares} shares on market ${marketId}`);
            return true;
        } catch (error) {
            console.error('Failed to sell shares:', error);
            return false;
        } finally {
            setIsLoading(false);
        }
    }, [isConnected]);

    const getMarket = useCallback((id: number): Market | undefined => {
        return markets.find(m => m.id === id);
    }, [markets]);

    return (
        <MarketContext.Provider value={{
            markets,
            userPositions,
            isLoading,
            createMarket,
            buyShares,
            sellShares,
            fetchMarkets,
            getMarket,
        }}>
            {children}
        </MarketContext.Provider>
    );
};

export const useMarkets = () => {
    const context = useContext(MarketContext);
    if (context === undefined) {
        throw new Error('useMarkets must be used within a MarketProvider');
    }
    return context;
};
