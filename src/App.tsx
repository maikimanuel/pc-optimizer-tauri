import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import Dashboard from './components/Dashboard'
import TweaksPanel from './components/TweaksPanel'
import Search from './components/Search'
import { Settings } from 'lucide-react'
import './App.css'

interface Tweak {
  id: string
  name: string
  category: string
  description: string
  enabled: boolean
}

function App() {
  const [tweaks, setTweaks] = useState<Tweak[]>([])
  const [filteredTweaks, setFilteredTweaks] = useState<Tweak[]>([])
  const [activeTab, setActiveTab] = useState<'dashboard' | 'tweaks' | 'backups'>('dashboard')
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadTweaks()
  }, [])

  const loadTweaks = async () => {
    try {
      const result = await invoke('get_tweaks')
      setTweaks(result as Tweak[])
      setFilteredTweaks(result as Tweak[])
    } catch (error) {
      console.error('Failed to load tweaks:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSearch = (query: string) => {
    const filtered = tweaks.filter(
      (t) =>
        t.name.toLowerCase().includes(query.toLowerCase()) ||
        t.description.toLowerCase().includes(query.toLowerCase()) ||
        t.category.toLowerCase().includes(query.toLowerCase())
    )
    setFilteredTweaks(filtered)
  }

  const handleApplyTweak = async (tweakId: string) => {
    try {
      await invoke('apply_tweak', { tweakId })
      loadTweaks()
    } catch (error) {
      console.error('Failed to apply tweak:', error)
    }
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <div className="header-content">
          <h1>⚡ PC Optimizer</h1>
          <p className="subtitle">Optimize and customize your Windows experience</p>
        </div>
        <nav className="app-nav">
          <button
            className={`nav-btn ${activeTab === 'dashboard' ? 'active' : ''}`}
            onClick={() => setActiveTab('dashboard')}
          >
            Dashboard
          </button>
          <button
            className={`nav-btn ${activeTab === 'tweaks' ? 'active' : ''}`}
            onClick={() => setActiveTab('tweaks')}
          >
            Tweaks
          </button>
          <button
            className={`nav-btn ${activeTab === 'backups' ? 'active' : ''}`}
            onClick={() => setActiveTab('backups')}
          >
            Backups
          </button>
          <button className="settings-btn">
            <Settings size={20} />
          </button>
        </nav>
      </header>

      <main className="app-main">
        {activeTab === 'dashboard' && <Dashboard />}
        {activeTab === 'tweaks' && (
          <div>
            <Search onSearch={handleSearch} />
            <TweaksPanel tweaks={filteredTweaks} loading={loading} onApply={handleApplyTweak} />
          </div>
        )}
        {activeTab === 'backups' && <div className="placeholder">Backups coming soon</div>}
      </main>
    </div>
  )
}

export default App
