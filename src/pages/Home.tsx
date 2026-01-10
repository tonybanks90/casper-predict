import React from 'react';
import { Link } from 'react-router-dom';
import { TrendingUp, ShieldCheck, Users } from 'lucide-react';
import { Button } from '../components/ui/button';
import { Card, CardHeader, CardTitle, CardContent } from '../components/ui/card';

const Home: React.FC = () => {
    return (
        <div className="space-y-16">
            {/* Hero Section */}
            <section className="text-center space-y-6 pt-12">
                <h1 className="text-5xl md:text-6xl font-extrabold bg-gradient-to-r from-casper-red to-blue-500 bg-clip-text text-transparent">
                    Predict the Future. Win.
                </h1>
                <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
                    The first decentralized prediction market on the Casper Network.
                    No liquidity pools, low fees, and winner-takes-all economics.
                </p>
                <div className="flex justify-center gap-4 pt-4">
                    <Button asChild size="lg" className="font-bold text-lg shadow-lg shadow-primary/20">
                        <Link to="/markets">Explore Markets</Link>
                    </Button>
                    <Button asChild variant="outline" size="lg" className="font-bold text-lg">
                        <Link to="/about">Learn More</Link>
                    </Button>
                </div>
            </section>

            {/* Stats Bar */}
            <section className="grid grid-cols-1 md:grid-cols-3 gap-8 py-8 border-y border-border bg-card/50">
                <div className="text-center">
                    <div className="text-3xl font-bold text-foreground">1.2M CSPR</div>
                    <div className="text-muted-foreground uppercase text-sm tracking-wider">Total Volume</div>
                </div>
                <div className="text-center">
                    <div className="text-3xl font-bold text-foreground">342</div>
                    <div className="text-muted-foreground uppercase text-sm tracking-wider">Active Markets</div>
                </div>
                <div className="text-center">
                    <div className="text-3xl font-bold text-foreground">5.6K</div>
                    <div className="text-muted-foreground uppercase text-sm tracking-wider">Traders</div>
                </div>
            </section>

            {/* Features */}
            <section className="grid md:grid-cols-3 gap-8">
                <Card className="bg-card/50 border-border">
                    <CardHeader>
                        <TrendingUp className="w-12 h-12 text-primary mb-2" />
                        <CardTitle>Bonding Curves</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p className="text-muted-foreground">Automatic price discovery and instant liquidity without the need for market makers.</p>
                    </CardContent>
                </Card>

                <Card className="bg-card/50 border-border">
                    <CardHeader>
                        <ShieldCheck className="w-12 h-12 text-blue-400 mb-2" />
                        <CardTitle>Secure & Transparent</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p className="text-muted-foreground">All trades and resolutions are verifiable on the Casper Network blockchain.</p>
                    </CardContent>
                </Card>

                <Card className="bg-card/50 border-border">
                    <CardHeader>
                        <Users className="w-12 h-12 text-purple-400 mb-2" />
                        <CardTitle>Winner Takes All</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p className="text-muted-foreground">Profit from your insights. Winners receive the stakes of those who guessed wrong.</p>
                    </CardContent>
                </Card>
            </section>
        </div>
    );
};

export default Home;
