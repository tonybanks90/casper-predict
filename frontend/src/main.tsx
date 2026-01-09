import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { WalletProvider } from './contexts/WalletContext.tsx'
import { MarketProvider } from './contexts/MarketContext.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WalletProvider>
      <MarketProvider>
        <App />
      </MarketProvider>
    </WalletProvider>
  </StrictMode>,
)
