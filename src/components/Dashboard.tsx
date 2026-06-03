import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Activity, HardDrive, Zap } from 'lucide-react'

interface SystemInfo {
  cpu_usage: number
  memory_usage: number
  memory_total: number
  disk_free: number
  disk_total: number
  uptime: number
}

export default function Dashboard() {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadSystemInfo()
    const interval = setInterval(loadSystemInfo, 2000)
    return () => clearInterval(interval)
  }, [])

  const loadSystemInfo = async () => {
    try {
      const info = await invoke('get_system_info')
      setSystemInfo(info as SystemInfo)
    } catch (error) {
      console.error('Failed to get system info:', error)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return <div className="dashboard-loading">Loading system information...</div>
  }

  if (!systemInfo) {
    return <div className="dashboard-error">Failed to load system information</div>
  }

  const memoryPercent = (systemInfo.memory_usage / systemInfo.memory_total) * 100
  const diskPercent = ((systemInfo.disk_total - systemInfo.disk_free) / systemInfo.disk_total) * 100

  return (
    <div className="dashboard">
      <div className="dashboard-grid">
        <div className="stat-card">
          <div className="stat-header">
            <Zap className="stat-icon" />
            <h3>CPU Usage</h3>
          </div>
          <div className="stat-value">{systemInfo.cpu_usage.toFixed(1)}%</div>
          <div className="stat-bar">
            <div className="stat-bar-fill" style={{ width: `${systemInfo.cpu_usage}%` }}></div>
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-header">
            <Activity className="stat-icon" />
            <h3>Memory Usage</h3>
          </div>
          <div className="stat-value">{memoryPercent.toFixed(1)}%</div>
          <div className="stat-bar">
            <div className="stat-bar-fill" style={{ width: `${memoryPercent}%` }}></div>
          </div>
          <div className="stat-detail">
            {(systemInfo.memory_usage / 1024 / 1024 / 1024).toFixed(1)} GB / {(systemInfo.memory_total / 1024 / 1024 / 1024).toFixed(1)} GB
          </div>
        </div>

        <div className="stat-card">
          <div className="stat-header">
            <HardDrive className="stat-icon" />
            <h3>Disk Usage</h3>
          </div>
          <div className="stat-value">{diskPercent.toFixed(1)}%</div>
          <div className="stat-bar">
            <div className="stat-bar-fill" style={{ width: `${diskPercent}%` }}></div>
          </div>
          <div className="stat-detail">
            {(systemInfo.disk_free / 1024 / 1024 / 1024).toFixed(1)} GB free
          </div>
        </div>
      </div>

      <div className="quick-actions">
        <h2>Quick Actions</h2>
        <div className="action-buttons">
          <button className="action-btn primary">🧹 Clean Temp Files</button>
          <button className="action-btn primary">🌐 Clean Browser Cache</button>
          <button className="action-btn secondary">📊 Generate Report</button>
        </div>
      </div>
    </div>
  )
}
