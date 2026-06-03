import React, { useState } from 'react'
import { CheckCircle, Circle } from 'lucide-react'

interface Tweak {
  id: string
  name: string
  category: string
  description: string
  enabled: boolean
}

interface Props {
  tweaks: Tweak[]
  loading: boolean
  onApply: (id: string) => void
}

const categoryColors: Record<string, string> = {
  privacy: '#ef4444',
  performance: '#3b82f6',
  gaming: '#8b5cf6',
  bloatware: '#f59e0b',
  system: '#10b981',
}

export default function TweaksPanel({ tweaks, loading, onApply }: Props) {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)

  const categories = Array.from(new Set(tweaks.map((t) => t.category)))
  const filteredTweaks = selectedCategory ? tweaks.filter((t) => t.category === selectedCategory) : tweaks

  if (loading) {
    return <div className="tweaks-loading">Loading tweaks...</div>
  }

  return (
    <div className="tweaks-panel">
      <div className="category-filter">
        <button
          className={`filter-btn ${selectedCategory === null ? 'active' : ''}`}
          onClick={() => setSelectedCategory(null)}
        >
          All Tweaks ({tweaks.length})
        </button>
        {categories.map((cat) => (
          <button
            key={cat}
            className={`filter-btn ${selectedCategory === cat ? 'active' : ''}`}
            onClick={() => setSelectedCategory(cat)}
            style={{
              borderLeftColor: categoryColors[cat] || '#6b7280',
            }}
          >
            {cat.charAt(0).toUpperCase() + cat.slice(1)} ({tweaks.filter((t) => t.category === cat).length})
          </button>
        ))}
      </div>

      <div className="tweaks-list">
        {filteredTweaks.map((tweak) => (
          <div key={tweak.id} className="tweak-item">
            <div className="tweak-header">
              <div className="tweak-title-section">
                <div className="category-badge" style={{ backgroundColor: categoryColors[tweak.category] || '#6b7280' }}>
                  {tweak.category}
                </div>
                <div>
                  <h3 className="tweak-name">{tweak.name}</h3>
                  <p className="tweak-description">{tweak.description}</p>
                </div>
              </div>
              <button className={`toggle-btn ${tweak.enabled ? 'enabled' : 'disabled'}`} onClick={() => onApply(tweak.id)}>
                {tweak.enabled ? <CheckCircle size={24} /> : <Circle size={24} />}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
