import React from 'react';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardContent, CardFooter } from '../components/ui/card';
import { Badge } from '../components/ui/badge';
import { useMarkets } from '../contexts/MarketContext';

const Markets: React.FC = () => {
    const { markets, isLoading } = useMarkets();

    const getCategoryColor = (category: string) => {
        const colors: Record<string, string> = {
            crypto: 'text-blue-400 bg-blue-500/10 hover:bg-blue-500/20',
            sports: 'text-green-400 bg-green-500/10 hover:bg-green-500/20',
            politics: 'text-purple-400 bg-purple-500/10 hover:bg-purple-500/20',
            weather: 'text-cyan-400 bg-cyan-500/10 hover:bg-cyan-500/20',
            tech: 'text-orange-400 bg-orange-500/10 hover:bg-orange-500/20',
        };
        return colors[category] || 'text-gray-400 bg-gray-500/10';
    };

    const formatTimeRemaining = (endTime: Date) => {
        const now = new Date();
        const diff = endTime.getTime() - now.getTime();
        const days = Math.floor(diff / (1000 * 60 * 60 * 24));
        if (days > 0) return `${days}d`;
        const hours = Math.floor(diff / (1000 * 60 * 60));
        if (hours > 0) return `${hours}h`;
        return 'Soon';
    };

    return (
        <div className="space-y-6">
            <div className="flex justify-between items-center">
                <h1 className="text-3xl font-bold tracking-tight">Explore Markets</h1>
                <div className="flex gap-2">
                    <select className="flex h-10 w-[150px] items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2">
                        <option>All Categories</option>
                        <option>Crypto</option>
                        <option>Sports</option>
                        <option>Politics</option>
                    </select>
                    <select className="flex h-10 w-[150px] items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2">
                        <option>Most Volume</option>
                        <option>Newest</option>
                        <option>Ending Soon</option>
                    </select>
                </div>
            </div>

            {isLoading ? (
                <div className="flex justify-center py-12">
                    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
                </div>
            ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {markets.map((market) => (
                        <Link key={market.id} to={`/market/${market.id}`}>
                            <Card className="hover:border-primary/50 transition-all cursor-pointer group h-full flex flex-col">
                                <CardHeader className="pb-2">
                                    <div className="flex justify-between items-start mb-2">
                                        <Badge variant="secondary" className={`font-bold uppercase ${getCategoryColor(market.category)}`}>
                                            {market.category}
                                        </Badge>
                                        <span className="text-xs text-muted-foreground">
                                            Ends in {formatTimeRemaining(market.endTime)}
                                        </span>
                                    </div>
                                    <h3 className="font-bold text-lg leading-tight group-hover:text-primary transition-colors">
                                        {market.question}
                                    </h3>
                                </CardHeader>
                                <CardContent className="pb-2 flex-grow">
                                    <div className="space-y-2">
                                        <div className="flex justify-between text-sm font-medium">
                                            <span className="text-green-500">YES {market.yesPercentage}%</span>
                                            <span className="text-red-500">NO {market.noPercentage}%</span>
                                        </div>
                                        <div className="h-2 bg-secondary rounded-full overflow-hidden flex">
                                            <div
                                                className="bg-green-500 transition-all"
                                                style={{ width: `${market.yesPercentage}%` }}
                                            ></div>
                                        </div>
                                    </div>
                                </CardContent>
                                <CardFooter className="pt-2 border-t text-sm text-muted-foreground flex justify-between">
                                    <span>Vol: {market.totalVolume.toLocaleString()} CSPR</span>
                                    <span className="text-primary font-semibold group-hover:translate-x-1 transition-transform">
                                        Trade →
                                    </span>
                                </CardFooter>
                            </Card>
                        </Link>
                    ))}
                </div>
            )}
        </div>
    );
};

export default Markets;
