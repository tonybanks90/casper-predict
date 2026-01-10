import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../components/ui/card';
import { Input } from '../components/ui/input';
import { Label } from '../components/ui/label';
import { useWallet } from '../contexts/WalletContext';
import { useMarkets } from '../contexts/MarketContext';

const CreateMarket: React.FC = () => {
    const navigate = useNavigate();
    const { isConnected, connect, isConnecting } = useWallet();
    const { createMarket, isLoading } = useMarkets();

    const [question, setQuestion] = useState('');
    const [category, setCategory] = useState('crypto');
    const [endDate, setEndDate] = useState('');
    const [initialLiquidity, setInitialLiquidity] = useState('10');
    const [marketType, setMarketType] = useState<'binary' | 'multiple_choice'>('binary');

    const handleCreate = async () => {
        if (!question || !endDate) {
            alert('Please fill in all fields');
            return;
        }

        const marketId = await createMarket({
            question,
            category,
            endTime: new Date(endDate),
            initialLiquidity: parseFloat(initialLiquidity),
            marketType,
        });

        if (marketId !== null) {
            alert(`Market created successfully! ID: ${marketId}`);
            navigate('/markets');
        } else {
            alert('Failed to create market');
        }
    };

    return (
        <div className="max-w-2xl mx-auto py-8">
            <div className="space-y-6">
                <div className="text-center space-y-2">
                    <h1 className="text-3xl font-bold tracking-tight">Create New Market</h1>
                    <p className="text-muted-foreground">Launch a prediction market on the Casper Network</p>
                </div>

                <Card>
                    <CardHeader>
                        <CardTitle>Market Details</CardTitle>
                        <CardDescription>Define the core parameters for your prediction market.</CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-6">
                        <div className="space-y-2">
                            <Label htmlFor="question">Question</Label>
                            <Input
                                id="question"
                                placeholder="e.g. Will BTC hit $100k by Dec 31?"
                                value={question}
                                onChange={(e) => setQuestion(e.target.value)}
                            />
                        </div>

                        <div className="space-y-2">
                            <Label htmlFor="type">Market Type</Label>
                            <select
                                id="type"
                                value={marketType}
                                onChange={(e) => setMarketType(e.target.value as 'binary' | 'multiple_choice')}
                                className="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                            >
                                <option value="binary">Binary (Yes/No)</option>
                                <option value="multiple_choice">Multiple Choice</option>
                            </select>
                        </div>

                        <div className="grid grid-cols-2 gap-4">
                            <div className="space-y-2">
                                <Label htmlFor="category">Category</Label>
                                <select
                                    id="category"
                                    value={category}
                                    onChange={(e) => setCategory(e.target.value)}
                                    className="flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                                >
                                    <option value="crypto">Crypto</option>
                                    <option value="sports">Sports</option>
                                    <option value="politics">Politics</option>
                                    <option value="weather">Weather</option>
                                    <option value="tech">Tech</option>
                                </select>
                            </div>
                            <div className="space-y-2">
                                <Label htmlFor="date">End Date</Label>
                                <Input
                                    id="date"
                                    type="datetime-local"
                                    value={endDate}
                                    onChange={(e) => setEndDate(e.target.value)}
                                />
                            </div>
                        </div>

                        <div className="space-y-2">
                            <Label htmlFor="liquidity">Initial Liquidity (CSPR)</Label>
                            <Input
                                id="liquidity"
                                type="number"
                                placeholder="10"
                                value={initialLiquidity}
                                onChange={(e) => setInitialLiquidity(e.target.value)}
                            />
                            <p className="text-xs text-muted-foreground">
                                Minimum: 1 CSPR. This creates the initial market pool.
                            </p>
                        </div>

                        {isConnected ? (
                            <Button
                                size="lg"
                                className="w-full"
                                onClick={handleCreate}
                                disabled={isLoading}
                            >
                                {isLoading ? "Creating..." : "Create Market"}
                            </Button>
                        ) : (
                            <Button size="lg" className="w-full" onClick={connect} disabled={isConnecting}>
                                {isConnecting ? "Connecting..." : "Connect Wallet to Create"}
                            </Button>
                        )}
                    </CardContent>
                </Card>
            </div>
        </div>
    );
};

export default CreateMarket;
