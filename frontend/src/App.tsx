
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Layout from './components/layout/Layout';
import Home from './pages/Home';
import Markets from './pages/Markets';
import MarketDetail from './pages/MarketDetail';
import Portfolio from './pages/Portfolio';
import CreateMarket from './pages/CreateMarket';
import AdminDashboard from './pages/AdminDashboard';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Home />} />
          <Route path="markets" element={<Markets />} />
          <Route path="market/:id" element={<MarketDetail />} />
          <Route path="portfolio" element={<Portfolio />} />
          <Route path="create" element={<CreateMarket />} />
          <Route path="admin" element={<AdminDashboard />} />
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
