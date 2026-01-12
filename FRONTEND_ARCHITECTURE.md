# Casper Predict - Frontend Architecture

## Tech Stack

**Framework:** React + TypeScript  
**Styling:** Tailwind CSS  
**State Management:** React Context API / Zustand  
**Blockchain Integration:** Casper SDK (`casper-js-sdk`)  
**Wallet:** Casper Wallet / Casper Signer integration  
**Charts:** Recharts (for bonding curve visualization)  
**Routing:** React Router v6  

---

## Page Structure

### 1. **Home / Landing Page** (`/`)

**Purpose:** First impression, explain concept, drive conversions

**Sections:**
```
┌─────────────────────────────────────────┐
│  HERO SECTION                           │
│  - Headline: "Predict the Future. Win." │
│  - Subheading: "First prediction market │
│    on Casper Network"                   │
│  - CTA: "Explore Markets" + "Connect"   │
│  - Live stats: Total volume, Markets,   │
│    Active users                         │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  FEATURED MARKETS (3-4 cards)           │
│  - Trending predictions                 │
│  - Live price movements                 │
│  - Quick "Trade Now" buttons            │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  HOW IT WORKS (3-step visual)           │
│  1. Connect Wallet                      │
│  2. Choose Prediction                   │
│  3. Buy Shares & Win                    │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  WHY CASPER PREDICT?                    │
│  - No liquidity pools                   │
│  - Winner takes all                     │
│  - Low fees (~$0.05)                    │
│  - Bonding curve pricing                │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  FOOTER                                 │
│  - Links: Docs, GitHub, Twitter         │
│  - Contract addresses                   │
└─────────────────────────────────────────┘
```

**Key Components:**
- `HeroSection.tsx`
- `FeaturedMarkets.tsx`
- `HowItWorks.tsx`
- `StatsBar.tsx` (Total Volume, # Markets, # Users)

---

### 2. **Markets Page** (`/markets`)

**Purpose:** Browse all prediction markets with filtering

**Layout:**
```
┌─────────────────────────────────────────┐
│  HEADER BAR                             │
│  [Search] [Filter▾] [Category▾] [Sort▾]│
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  FILTER SIDEBAR (Desktop) / Drawer (M)  │
│  Categories:                            │
│    ☐ Crypto  ☐ Sports  ☐ Politics      │
│    ☐ Entertainment  ☐ Other            │
│  Status:                                │
│    ☐ Active  ☐ Closed  ☐ Resolved      │
│  Market Type:                           │
│    ☐ Binary  ☐ Multiple  ☐ Compound    │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  MARKET GRID                            │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐│
│  │ Market 1 │ │ Market 2 │ │ Market 3 ││
│  │ Card     │ │ Card     │ │ Card     ││
│  └──────────┘ └──────────┘ └──────────┘│
│  ┌──────────┐ ┌──────────┐ ┌──────────┐│
│  │ Market 4 │ │ Market 5 │ │ Market 6 ││
│  └──────────┘ └──────────┘ └──────────┘│
│  [Load More]                            │
└─────────────────────────────────────────┘
```

**Market Card Component:**
```
┌─────────────────────────────────────┐
│ 🏀 SPORTS                           │
│ Will Lakers win NBA Finals 2026?   │
│                                     │
│ YES 67%  ━━━━━━━━━━━━━━▒▒▒▒▒  NO 33%│
│                                     │
│ 💰 Total Pool: 1,234 CSPR          │
│ ⏰ Ends: Jan 15, 2026              │
│ 👥 234 traders                      │
│                                     │
│ [Trade Now →]                       │
└─────────────────────────────────────┘
```

**Key Components:**
- `MarketCard.tsx`
- `MarketFilters.tsx`
- `SearchBar.tsx`
- `MarketGrid.tsx`
- `CategoryTabs.tsx`

**Filters:**
- Category (Crypto, Sports, Politics, Entertainment)
- Status (Active, Closed, Resolved)
- Market Type (Binary, Multiple Choice, Compound)
- Time Range (Ends Soon, This Week, This Month)

**Sorting:**
- Most Popular (by volume)
- Newest
- Ending Soon
- Highest Volume

---

### 3. **Market Detail Page** (`/market/:id`)

**Purpose:** Deep dive into single market, trading interface

**Layout:**
```
┌──────────────────────────────────────────────────┐
│  BREADCRUMB: Markets > Sports > This Market     │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  MARKET HEADER                                   │
│  🏀 SPORTS                                       │
│  Will Lakers win NBA Finals 2026?               │
│  Status: Active ● Ends: Jan 15, 2026            │
│  Created by: 0x123...abc  Resolution: Admin     │
└──────────────────────────────────────────────────┘

┌──────────────┬───────────────────────────────────┐
│  LEFT PANEL  │  RIGHT PANEL (Trading)            │
│              │                                   │
│ OUTCOMES     │  BUY SHARES                       │
│ ┌──────────┐ │  Outcome: [YES ▾]                │
│ │YES   67% │ │  Shares:  [____] (≈ 10.5 CSPR)   │
│ │1.01 CSPR │ │  Price:   1.01 CSPR/share        │
│ └──────────┘ │  Slippage: [1%▾]                 │
│ ┌──────────┐ │  Total:   10.50 CSPR             │
│ │NO    33% │ │  [Buy Shares] 💰                 │
│ │0.45 CSPR │ │                                   │
│ └──────────┘ │  ─────────────────               │
│              │  YOUR POSITION                    │
│ CHART        │  YES: 25 shares (1.5% of pool)   │
│ [Bonding     │  Value: 25.25 CSPR               │
│  Curve       │  Unrealized P/L: +2.5 CSPR       │
│  Graph]      │  [Sell Shares]                    │
│              │                                   │
│ DETAILS      │  ─────────────────               │
│ • Pool: 1234 │  MARKET STATS                     │
│ • Traders:234│  Total Volume: 5,678 CSPR        │
│ • Volume: 5K │  # Traders: 234                   │
│ • Fee: 2%    │  Liquidity: 1,234 CSPR           │
└──────────────┴───────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  TABS: [Activity] [Your Trades] [Info]          │
│  ─────────────────────────────────────────────── │
│  ACTIVITY FEED                                   │
│  🟢 Alice bought 50 YES @ 1.02 CSPR - 2m ago    │
│  🔴 Bob sold 30 NO @ 0.44 CSPR - 5m ago         │
│  🟢 Charlie bought 20 YES @ 1.00 CSPR - 8m ago  │
└──────────────────────────────────────────────────┘
```

**Key Components:**
- `MarketHeader.tsx`
- `OutcomeCards.tsx`
- `TradingPanel.tsx`
- `BondingCurveChart.tsx` (Recharts)
- `MarketStats.tsx`
- `ActivityFeed.tsx`
- `UserPosition.tsx`

**Trading Flow:**
1. Select outcome (YES/NO or option A/B/C)
2. Enter number of shares OR CSPR amount
3. Auto-calculate total cost from bonding curve
4. Set slippage tolerance (0.5%, 1%, 2%, 5%)
5. Preview transaction (shares, price, fees)
6. Click "Buy Shares" → Casper Wallet popup
7. Confirm transaction → Loading state → Success toast
8. Update UI with new position

---

### 4. **Portfolio Page** (`/portfolio`)

**Purpose:** User's positions, history, winnings

**Layout:**
```
┌──────────────────────────────────────────────────┐
│  PORTFOLIO OVERVIEW                              │
│  Connected: 0x123...abc                         │
│  ┌────────────┬────────────┬────────────┐       │
│  │ Total Value│ Total P/L  │ Markets    │       │
│  │ 1,234 CSPR │ +234 CSPR  │ 12 Active  │       │
│  │            │ (+23.4%)   │            │       │
│  └────────────┴────────────┴────────────┘       │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  TABS: [Active Positions] [History] [Winnings]  │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  ACTIVE POSITIONS                                │
│  ┌────────────────────────────────────────────┐ │
│  │ Market: Will Bitcoin reach $100K?         │ │
│  │ Outcome: YES • 50 shares @ avg 0.95 CSPR  │ │
│  │ Current Price: 1.10 CSPR                  │ │
│  │ Value: 55 CSPR • P/L: +7.5 CSPR (+15.8%)  │ │
│  │ [View Market] [Sell]                      │ │
│  └────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────┐ │
│  │ Market: Lakers to win NBA?                │ │
│  │ ... (similar card)                        │ │
│  └────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  CLAIMABLE WINNINGS                              │
│  ┌────────────────────────────────────────────┐ │
│  │ ✅ BTC reached $100K (Resolved: YES)      │ │
│  │ Your Position: 50 YES shares              │ │
│  │ Payout: 125 CSPR (🎉 +62.5 CSPR profit)  │ │
│  │ [Claim Winnings 💰]                       │ │
│  └────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

**Key Components:**
- `PortfolioOverview.tsx`
- `PositionCard.tsx`
- `TransactionHistory.tsx`
- `ClaimableWinnings.tsx`
- `ProfitLossChart.tsx`

---

### 5. **Create Market Page** (`/create`)

**Purpose:** Admin/users create new prediction markets

**Layout:**
```
┌──────────────────────────────────────────────────┐
│  CREATE NEW MARKET                               │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  Step 1 of 3: Market Type                       │
└──────────────────────────────────────────────────┘

┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│   BINARY    │ │  MULTIPLE   │ │  COMPOUND   │
│   YES/NO    │ │   CHOICE    │ │   COMPLEX   │
│ [Selected ✓]│ │             │ │             │
└─────────────┘ └─────────────┘ └─────────────┘

┌──────────────────────────────────────────────────┐
│  Step 2 of 3: Market Details                    │
│                                                  │
│  Question *                                      │
│  [___________________________________________]   │
│  Example: "Will Bitcoin reach $100K by Jan?"    │
│                                                  │
│  Category *                                      │
│  [Crypto ▾]                                     │
│                                                  │
│  End Date & Time *                              │
│  [Jan 15, 2026] [11:59 PM]                      │
│                                                  │
│  Resolution Source                               │
│  [___________________________________________]   │
│  Example: "CoinGecko API" or "Admin decision"   │
│                                                  │
│  Description (Optional)                          │
│  [                                             ] │
│  [                                             ] │
│                                                  │
│  Initial Liquidity *                             │
│  [100] CSPR (minimum: 50 CSPR)                  │
│                                                  │
│  Bonding Curve Parameters (Advanced ▾)          │
│  Initial Price: [0.01] CSPR                     │
│  Curve Steepness (k): [0.001]                   │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  Step 3 of 3: Review & Deploy                   │
│                                                  │
│  Preview your market:                            │
│  Question: Will Bitcoin reach $100K by Jan?     │
│  Type: Binary (YES/NO)                          │
│  Category: Crypto                                │
│  Ends: Jan 15, 2026 11:59 PM                    │
│  Initial Liquidity: 100 CSPR                    │
│  Estimated Gas: ~50 CSPR (~$0.27)               │
│                                                  │
│  ☐ I understand resolution is my responsibility │
│                                                  │
│  [← Back]  [Create Market 🚀]                   │
└──────────────────────────────────────────────────┘
```

**Key Components:**
- `MarketTypeSelector.tsx`
- `MarketDetailsForm.tsx`
- `OutcomeBuilder.tsx` (for multiple choice)
- `BondingCurveConfig.tsx`
- `MarketPreview.tsx`

---

### 6. **Admin Dashboard** (`/admin`)

**Purpose:** Resolve markets, manage platform (admin only)

**Layout:**
```
┌──────────────────────────────────────────────────┐
│  ADMIN DASHBOARD                                 │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  Platform Stats                                  │
│  Total Markets: 156 • Volume: 123,456 CSPR      │
│  Fees Collected: 2,469 CSPR                     │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  PENDING RESOLUTIONS                             │
│  ┌────────────────────────────────────────────┐ │
│  │ Market #42: Will Lakers win?              │ │
│  │ Ended: Jan 10, 2026 • Status: Closed     │ │
│  │ Pool: 1,234 CSPR • 234 traders           │ │
│  │                                            │ │
│  │ Resolve as: [YES ▾]                       │ │
│  │ Proof URL: [________________________]     │ │
│  │ [Resolve Market] [Cancel Market]          │ │
│  └────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────┐
│  PLATFORM CONTROLS                               │
│  Fee Rate: [2%]  [Update]                       │
│  Emergency Pause: [Pause Platform]               │
│  Claim Fees: 2,469 CSPR [Claim to Wallet]      │
└──────────────────────────────────────────────────┘
```

**Key Components:**
- `AdminStats.tsx`
- `PendingResolutions.tsx`
- `ResolveMarketForm.tsx`
- `PlatformControls.tsx`

---

## Global Components

### Header / Navbar
```
┌──────────────────────────────────────────────────┐
│ 🎲 CasperPredict  [Markets] [Portfolio] [Create]│
│                                   [Connect Wallet]│
└──────────────────────────────────────────────────┘

// When connected:
│                           [0x123...abc ▾] [12.5Ⓒ]│
//                           └─ Dropdown menu:
//                              • Portfolio
//                              • Settings
//                              • Disconnect
```

**Component:** `Navbar.tsx`

### Footer
```
┌──────────────────────────────────────────────────┐
│ CasperPredict © 2026                            │
│ [Docs] [GitHub] [Twitter] [Discord]             │
│ Contracts: 0xFactory... • 0xVault...            │
│ Built on Casper Network                          │
└──────────────────────────────────────────────────┘
```

**Component:** `Footer.tsx`

### Modals/Dialogs
- `ConnectWalletModal.tsx` - Casper Wallet / Casper Signer
- `TransactionModal.tsx` - Loading, Success, Error states
- `ConfirmTradeModal.tsx` - Review before buy/sell

### Notifications
- `Toast.tsx` - Success/error notifications (use `react-hot-toast`)

---

## User Flows

### Flow 1: New User Trading
```
1. Land on Homepage
   ↓
2. Click "Explore Markets"
   ↓
3. Browse Markets page
   ↓
4. Click on interesting market
   ↓
5. View Market Detail page
   ↓
6. Click "Connect Wallet" (if not connected)
   ↓
7. Casper Wallet popup → Approve
   ↓
8. Select outcome (YES)
   ↓
9. Enter shares (e.g., 10)
   ↓
10. See auto-calculated cost (10.5 CSPR)
   ↓
11. Click "Buy Shares"
   ↓
12. Confirm in Casper Wallet
   ↓
13. Transaction pending → Success toast
   ↓
14. See updated position in "Your Position" panel
```

### Flow 2: Claiming Winnings
```
1. User receives notification (email/push) - Market resolved
   ↓
2. Visit Portfolio page
   ↓
3. See "Claimable Winnings" section
   ↓
4. Click "Claim Winnings" button
   ↓
5. Casper Wallet confirmation
   ↓
6. Success → CSPR transferred to wallet
   ↓
7. Position moved to "History" tab
```

### Flow 3: Creating Market (Admin)
```
1. Navigate to /create
   ↓
2. Select Binary market type
   ↓
3. Fill in question, category, end date
   ↓
4. Set initial liquidity (100 CSPR)
   ↓
5. Review preview
   ↓
6. Click "Create Market"
   ↓
7. Casper Wallet approves deployment
   ↓
8. Market created → Redirect to market page
```

### Flow 4: Resolving Market (Admin)
```
1. Go to Admin Dashboard
   ↓
2. View "Pending Resolutions"
   ↓
3. Select market to resolve
   ↓
4. Choose winning outcome
   ↓
5. Add proof URL
   ↓
6. Click "Resolve Market"
   ↓
7. Casper Wallet confirmation
   ↓
8. Market resolved → Users can claim
```

---

## State Management Structure

### Global State (Zustand/Context)

```typescript
interface AppState {
  // User
  user: {
    address: string | null;
    balance: number;
    connected: boolean;
  };
  
  // Markets
  markets: Market[];
  activeMarket: Market | null;
  
  // User Positions
  positions: Position[];
  claimableWinnings: Winning[];
  
  // Platform
  platformStats: {
    totalVolume: number;
    totalMarkets: number;
    activeUsers: number;
  };
  
  // UI
  isLoading: boolean;
  error: string | null;
}

// Actions
connectWallet()
disconnectWallet()
fetchMarkets(filters)
fetchMarketById(id)
buyShares(marketId, outcomeId, shares)
sellShares(marketId, outcomeId, shares)
claimWinnings(marketId)
createMarket(data)
resolveMarket(marketId, outcome)
createMarket(data)
resolveMarket(marketId, outcome)
```

---

## Key Features by Component

### TradingPanel.tsx
- Real-time bonding curve price calculation
- Slippage protection settings
- Buy/Sell toggle
- Input validation (min/max amounts)
- Transaction preview
- Gas estimation

### BondingCurveChart.tsx
- Line chart showing price vs. supply
- Current position indicator
- Predicted price after trade
- Historical price overlay
- Interactive tooltips

### MarketCard.tsx
- Outcome probability bars
- Live price updates
- Time remaining countdown
- Quick stats (volume, traders)
- Status badge (Active/Closed/Resolved)

### ActivityFeed.tsx
- Real-time trade updates (via events)
- User avatars (blockie/identicon)
- Trade direction indicators (🟢 buy, 🔴 sell)
- Relative timestamps ("2m ago")
- Pagination/infinite scroll

---

## Responsive Design

**Mobile (<768px):**
- Hamburger menu for navigation
- Stacked layout (no sidebar)
- Bottom sheet for trading panel
- Simplified charts
- Touch-optimized buttons

**Tablet (768px-1024px):**
- Collapsible sidebar
- Two-column market grid
- Adapted trading panel

**Desktop (>1024px):**
- Full three-column layout
- Expanded charts
- Persistent sidebar
- Keyboard shortcuts

---

## Performance Optimizations

1. **Lazy Loading:** Code-split routes with `React.lazy()`
2. **Virtualization:** For long lists (markets, trades) use `react-window`
3. **Debouncing:** Search and filter inputs
4. **Caching:** Store market data with SWR or React Query
5. **Optimistic Updates:** Update UI before blockchain confirmation
6. **WebSocket:** Real-time price updates (instead of polling)

---

## Accessibility

- Semantic HTML (`<nav>`, `<main>`, `<article>`)
- ARIA labels for interactive elements
- Keyboard navigation support (Tab, Enter, Esc)
- Focus indicators on all inputs/buttons
- Screen reader announcements for trade confirmations
- Color contrast ratios (WCAG AA)

---

## Summary

**6 Pages:** Home, Markets, Market Detail, Portfolio, Create, Admin  
**~25 Components:** Modular, reusable, well-tested  
**4 User Flows:** Trading, Claiming, Creating, Resolving  
**Responsive:** Mobile-first, tablet-optimized, desktop-enhanced  

This architecture provides a complete, production-ready frontend for Casper Predict that's intuitive for users, powerful for traders, and maintainable for developers! 🚀
