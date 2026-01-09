import React from 'react';
import { Link } from 'react-router-dom';
import { Menu, X, Rocket, LayoutDashboard, Wallet, PlusCircle, LogOut } from 'lucide-react';
import clsx from 'clsx';
import { useWallet } from '../../contexts/WalletContext'; // Import the hook
import { Button } from '../ui/button';

const Navbar: React.FC = () => {
    const [isOpen, setIsOpen] = React.useState(false);
    const { isConnected, connect, disconnect, activeKey, isConnecting } = useWallet(); // Use context

    const toggleMenu = () => setIsOpen(!isOpen);

    const formatAddress = (key: string) => {
        return `${key.slice(0, 5)}...${key.slice(-5)}`;
    };

    const NavLink = ({ to, children, icon: Icon }: { to: string; children: React.ReactNode; icon?: any }) => (
        <Link
            to={to}
            className={clsx(
                "flex items-center space-x-2 px-4 py-2 rounded-lg transition-colors hover:bg-gray-800",
                "text-gray-300 hover:text-white"
            )}
            onClick={() => setIsOpen(false)}
        >
            {Icon && <Icon className="w-4 h-4" />}
            <span>{children}</span>
        </Link>
    );

    return (
        <nav className="bg-background border-b border-border sticky top-0 z-50">
            <div className="container mx-auto px-4">
                <div className="flex justify-between items-center h-16">
                    {/* Logo */}
                    <Link to="/" className="flex items-center space-x-2 font-bold text-xl text-primary hover:text-primary/80 transition-colors">
                        <Rocket className="w-6 h-6" />
                        <span>CasperPredict</span>
                    </Link>

                    {/* Desktop Nav */}
                    <div className="hidden md:flex items-center space-x-4">
                        <NavLink to="/markets" icon={LayoutDashboard}>Markets</NavLink>
                        <NavLink to="/portfolio" icon={Wallet}>Portfolio</NavLink>
                        <NavLink to="/create" icon={PlusCircle}>Create</NavLink>
                    </div>

                    {/* Connect Button */}
                    <div className="hidden md:flex items-center space-x-4">
                        {isConnected && activeKey ? (
                            <div className="flex items-center space-x-4">
                                <div className="px-4 py-2 rounded-lg bg-secondary text-secondary-foreground font-mono text-sm border border-border">
                                    {formatAddress(activeKey)}
                                </div>
                                <Button variant="ghost" size="icon" onClick={disconnect} title="Disconnect">
                                    <LogOut className="w-4 h-4" />
                                </Button>
                            </div>
                        ) : (
                            <Button
                                onClick={connect}
                                disabled={isConnecting}
                                className="shadow-lg shadow-primary/20"
                            >
                                {isConnecting ? "Connecting..." : "Connect Wallet"}
                            </Button>
                        )}
                    </div>

                    {/* Mobile Menu Button */}
                    <div className="md:hidden">
                        <button onClick={toggleMenu} className="text-gray-300 hover:text-white p-2">
                            {isOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
                        </button>
                    </div>
                </div>
            </div>

            {/* Mobile Menu */}
            {isOpen && (
                <div className="md:hidden bg-background border-b border-border py-4 px-4 space-y-2">
                    <NavLink to="/markets" icon={LayoutDashboard}>Markets</NavLink>
                    <NavLink to="/portfolio" icon={Wallet}>Portfolio</NavLink>
                    <NavLink to="/create" icon={PlusCircle}>Create</NavLink>
                    <div className="pt-4">
                        {isConnected && activeKey ? (
                            <div className="space-y-2">
                                <div className="w-full px-4 py-2 rounded-lg bg-secondary text-secondary-foreground font-mono text-sm text-center">
                                    {formatAddress(activeKey)}
                                </div>
                                <Button variant="destructive" className="w-full" onClick={disconnect}>
                                    Disconnect
                                </Button>
                            </div>
                        ) : (
                            <Button onClick={connect} disabled={isConnecting} className="w-full">
                                {isConnecting ? "Connecting..." : "Connect Wallet"}
                            </Button>
                        )}
                    </div>
                </div>
            )}
        </nav>
    );
};

export default Navbar;
