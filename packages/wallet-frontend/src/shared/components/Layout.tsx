import React from 'react';

interface LayoutProps {
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <div className="min-h-screen bg-gray-950 text-white">
      <header className="border-b border-gray-800 px-6 py-4 flex items-center gap-3">
        <span className="text-yellow-400 text-2xl">⚡</span>
        <h1 className="text-xl font-bold">Flash Wallet</h1>
        <span className="text-gray-500 text-sm ml-auto">Bénin 🇧🇯</span>
      </header>
      <main className="max-w-2xl mx-auto px-4 py-8">
        {children}
      </main>
    </div>
  );
};
