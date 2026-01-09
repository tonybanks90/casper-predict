import React from 'react';
import { Twitter, Github, Globe } from 'lucide-react';

const Footer: React.FC = () => {
    return (
        <footer className="bg-gray-950 border-t border-gray-800 py-12 mt-auto">
            <div className="container mx-auto px-4">
                <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
                    <div className="space-y-4">
                        <h3 className="font-bold text-lg text-white">CasperPredict</h3>
                        <p className="text-gray-400 text-sm">
                            The first decentralized prediction market on the Casper Network. Predict the future, win rewards.
                        </p>
                    </div>

                    <div>
                        <h4 className="font-semibold text-white mb-4">Platform</h4>
                        <ul className="space-y-2 text-sm text-gray-400">
                            <li><a href="/markets" className="hover:text-blue-400">Browse Markets</a></li>
                            <li><a href="/create" className="hover:text-blue-400">Create Market</a></li>
                            <li><a href="/portfolio" className="hover:text-blue-400">Portfolio</a></li>
                        </ul>
                    </div>

                    <div>
                        <h4 className="font-semibold text-white mb-4">Resources</h4>
                        <ul className="space-y-2 text-sm text-gray-400">
                            <li><a href="#" className="hover:text-blue-400">Documentation</a></li>
                            <li><a href="#" className="hover:text-blue-400">Help Center</a></li>
                            <li><a href="#" className="hover:text-blue-400">Terms of Service</a></li>
                        </ul>
                    </div>

                    <div>
                        <h4 className="font-semibold text-white mb-4">Community</h4>
                        <div className="flex space-x-4">
                            <a href="#" className="text-gray-400 hover:text-blue-400"><Twitter className="w-5 h-5" /></a>
                            <a href="#" className="text-gray-400 hover:text-white"><Github className="w-5 h-5" /></a>
                            <a href="#" className="text-gray-400 hover:text-blue-400"><Globe className="w-5 h-5" /></a>
                        </div>
                    </div>
                </div>

                <div className="border-t border-gray-800 mt-8 pt-8 text-center text-sm text-gray-500">
                    <p>© 2026 CasperPredict. Built on Casper Network.</p>
                </div>
            </div>
        </footer>
    );
};

export default Footer;
