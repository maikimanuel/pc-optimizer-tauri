import React, { useState } from 'react'
import { Search as SearchIcon } from 'lucide-react'

interface Props {
  onSearch: (query: string) => void
}

export default function Search({ onSearch }: Props) {
  const [query, setQuery] = useState('')

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setQuery(value)
    onSearch(value)
  }

  return (
    <div className="search-container">
      <div className="search-input-wrapper">
        <SearchIcon className="search-icon" size={20} />
        <input
          type="text"
          className="search-input"
          placeholder="Search tweaks..."
          value={query}
          onChange={handleChange}
        />
      </div>
    </div>
  )
}
