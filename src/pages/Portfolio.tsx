import React from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { useWallet } from '../contexts/WalletContext';

const Portfolio: React.FC = () => {
    const { activeKey, isConnected, balance, connect, isConnecting } = useWallet();

    if (!isConnected) {
        return (
            <div className="flex flex-col items-center justify-center min-h-[50vh] space-y-4">
                <h1 className="text-3xl font-bold">Portfolio</h1>
                <p className="text-muted-foreground">Please connect your wallet to view your portfolio.</p>
                <Button onClick={connect} disabled={isConnecting}>
                    {isConnecting ? "Connecting..." : "Connect Wallet"}
                </Button>
            </div>
        );
    }

    return (
        <div className="space-y-8">
            <h1 className="text-3xl font-bold">Your Portfolio</h1>

            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Balance
                        </CardTitle>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth="2"
                            className="h-4 w-4 text-muted-foreground"
                        >
                            <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
                        </svg>
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">
                            {balance !== null ? `${balance.toLocaleString()} CSPR` : "Loading..."}
                        </div>
                        <p className="text-xs text-muted-foreground">
                            Available on Testnet
                        </p>
                    </CardContent>
                </Card>
            </div>

            <Card>
                <CardHeader>
                    <CardTitle>Account Details</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="space-y-1">
                        <p className="text-sm font-medium leading-none">Public Key</p>
                        <p className="text-sm text-muted-foreground break-all font-mono bg-muted p-2 rounded-md">
                            {activeKey}
                        </p>
                    </div>
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardTitle>My Positions</CardTitle>
                </CardHeader>
                <CardContent>
                    <p className="text-muted-foreground">You don't have any active positions yet.</p>
                </CardContent>
            </Card>
        </div>
    );
};

export default Portfolio;
