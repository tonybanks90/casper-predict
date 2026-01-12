import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';
import { Badge } from '../components/ui/badge';
import { Input } from '../components/ui/input';
import { Label } from '../components/ui/label';
import { useWallet } from '../contexts/WalletContext';
import { useMarkets } from '../contexts/MarketContext';

const MarketDetail: React.FC = () => {
    const { id } = useParams();
    const marketId = parseInt(id || '0');
    const { isConnected, connect, isConnecting, balance } = useWallet();
    const { getMarket, buyShares, isLoading } = useMarkets();

    const [selectedOutcome, setSelectedOutcome] = useState<number>(0);
    const [amount, setAmount] = useState<string>('');

    const market = getMarket(marketId);

    if (!market) {
        return (
            <div className="flex justify-center items-center py-20">
                <p className="text-muted-foreground">Market not found</p>
            </div>
        );
    }

    const handleBuy = async () => {
        const cspr = parseFloat(amount);
        if (!cspr || cspr <= 0) {
            alert('Please enter a valid amount');
            return;
        }

        const success = await buyShares(marketId, selectedOutcome, cspr);
        if (success) {
            alert(`Successfully bought ${cspr} CSPR worth of ${selectedOutcome === 0 ? 'YES' : 'NO'} shares!`);
            setAmount('');
        } else {
            alert('Transaction failed');
        }
    };

    const getCategoryColor = () => {
        const colors: Record<string, string> = {
            crypto: 'text-blue-400 bg-blue-500/10',
            sports: 'text-green-400 bg-green-500/10',
            politics: 'text-purple-400 bg-purple-500/10',
            weather: 'text-cyan-400 bg-cyan-500/10',
            tech: 'text-orange-400 bg-orange-500/10',
        };
        return colors[market.category] || 'text-gray-400 bg-gray-500/10';
    };

    const estimatedShares = amount ? (parseFloat(amount) * 0.98).toFixed(2) : '0.00';
    const fees = amount ? (parseFloat(amount) * 0.02).toFixed(4) : '0.00';

    return (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            {/* Left Column: Info & Chart */}
            <div className="lg:col-span-2 space-y-6">
                <Card className="border-border/50">
                    <CardHeader>
                        <div className="flex items-center gap-3 mb-2">
                            <Badge variant="secondary" className={`font-bold uppercase ${getCategoryColor()}`}>
                                {market.category}
                            </Badge>
                            <span className="text-sm text-muted-foreground">Market #{market.id}</span>
                        </div>
                        <CardTitle className="text-2xl md:text-3xl font-bold leading-tight">
                            {market.question}
                        </CardTitle>
                        <CardDescription className="flex gap-6 text-sm pt-2">
                            <span>Ends: <span className="text-foreground font-medium">{market.endTime.toLocaleDateString()}</span></span>
                            <span>Volume: <span className="text-foreground font-medium">{market.totalVolume.toLocaleString()} CSPR</span></span>
                            <span>Created by: <span className="text-foreground font-medium">{market.creator}</span></span>
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="space-y-4">
                            <div className="flex justify-between text-sm font-medium">
                                <span className="text-green-500">YES {market.yesPercentage}%</span>
                                <span className="text-red-500">NO {market.noPercentage}%</span>
                            </div>
                            <div className="h-4 bg-secondary rounded-full overflow-hidden flex">
                                <div
                                    className="bg-green-500 transition-all"
                                    style={{ width: `${market.yesPercentage}%` }}
                                ></div>
                            </div>

                            <div className="h-64 bg-secondary/20 rounded-xl flex items-center justify-center border border-dashed border-border mt-6">
                                <span className="text-muted-foreground">Bonding Curve Chart Visualization</span>
                            </div>
                        </div>
                    </CardContent>
                </Card>
            </div>

            {/* Right Column: Trading */}
            <div className="space-y-6">
                <Card className="sticky top-24 border-border/50 shadow-lg">
                    <CardHeader>
                        <CardTitle>Trade Shares</CardTitle>
                        {balance !== null && (
                            <CardDescription>
                                Balance: <span className="font-mono text-foreground">{balance.toFixed(2)} CSPR</span>
                            </CardDescription>
                        )}
                    </CardHeader>
                    <CardContent className="space-y-6">
                        <div className="grid grid-cols-2 gap-4">
                            <button
                                onClick={() => setSelectedOutcome(0)}
                                className={`p-4 rounded-xl border transition-all text-center ${selectedOutcome === 0
                                    ? 'bg-green-500/20 border-green-500 ring-2 ring-green-500/20'
                                    : 'bg-card border-border hover:bg-accent'
                                    }`}
                            >
                                <div className="text-green-500 font-bold text-lg">YES</div>
                                <div className="text-sm text-muted-foreground">{market.yesPercentage}%</div>
                            </button>
                            <button
                                onClick={() => setSelectedOutcome(1)}
                                className={`p-4 rounded-xl border transition-all text-center ${selectedOutcome === 1
                                    ? 'bg-red-500/20 border-red-500 ring-2 ring-red-500/20'
                                    : 'bg-card border-border hover:bg-accent'
                                    }`}
                            >
                                <div className="text-red-500 font-bold text-lg">NO</div>
                                <div className="text-sm text-muted-foreground">{market.noPercentage}%</div>
                            </button>
                        </div>

                        <div className="space-y-4">
                            <div className="space-y-2">
                                <Label>Amount (CSPR)</Label>
                                <Input
                                    type="number"
                                    placeholder="0.00"
                                    value={amount}
                                    onChange={(e) => setAmount(e.target.value)}
                                />
                            </div>

                            <div className="space-y-1 pt-2">
                                <div className="flex justify-between text-sm">
                                    <span className="text-muted-foreground">Est. Shares</span>
                                    <span className="font-mono">{estimatedShares}</span>
                                </div>
                                <div className="flex justify-between text-sm">
                                    <span className="text-muted-foreground">Fees (2%)</span>
                                    <span className="font-mono">{fees} CSPR</span>
                                </div>
                            </div>

                            {isConnected ? (
                                <Button
                                    size="lg"
                                    className="w-full font-bold"
                                    onClick={handleBuy}
                                    disabled={isLoading || !amount}
                                >
                                    {isLoading ? "Processing..." : `Buy ${selectedOutcome === 0 ? 'YES' : 'NO'}`}
                                </Button>
                            ) : (
                                <Button size="lg" className="w-full" onClick={connect} disabled={isConnecting}>
                                    {isConnecting ? "Connecting..." : "Connect Wallet to Trade"}
                                </Button>
                            )}
                        </div>
                    </CardContent>
                </Card>
            </div>
        </div>
    );
};

export default MarketDetail;
