import { useState, useEffect } from 'react'
import { EmptySearch } from './Empty'
import { useQueryState } from 'nuqs'
import Conjugation from './Conjugation'
import SearchInput from './SearchInput'
import useSWR from 'swr'

export default function Search () {
  const [search, setSearch] = useQueryState('q')
  const [selectedIndex, setSelectedIndex] = useState(-1)

  const { data } = useSWR(`/api/search?q=${search}`, fetcher)
  const hasData = data && search && data.length > 1
  const hasSearchResults = data && search && data.length > 0

  // Reset selected index when search changes
  useEffect(() => {
    setSelectedIndex(-1)
  }, [search])

  const handleKeyDown = (e) => {
    if (!hasData) return

    if (e.key === 'ArrowDown') {
      e.preventDefault()
      setSelectedIndex(prev => {
        if (prev < data.length - 1) {
          return prev + 1
        }
        // Wrap around to 0 if at the end
        return 0
      })
    } else if (e.key === 'ArrowUp') {
      e.preventDefault()
      setSelectedIndex(prev => {
        if (prev === -1) {
          // Start from the last index if no item is selected
          return data.length - 1
        }
        if (prev > 0) {
          return prev - 1
        }
        return data.length - 1
      })
    } else if (e.key === 'Enter' && selectedIndex >= 0 && selectedIndex < data.length) {
      e.preventDefault()
      setSearch(data[selectedIndex].verb)
    }
  }

  const handleItemClick = (item) => {
    setSearch(item.verb)
  }

  const fullMatch = () => {
    if (!data) return null

    if (data.length === 1) {
      return data[0]
    }

    const item = data.find(item => item.verb === search)
    return item
  }

  const selectedVerb = fullMatch()

  return (
    <div>
      <h1 className='text-xs border-b border-gray-200 pb-1 w-md px-2 text-gray-500'>Verb</h1>
      <SearchInput
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        onKeyDown={handleKeyDown}
      />

      <div className='mt-2 max-w-md'>
        {hasData && !selectedVerb && (
          <ul className='divide-y divide-gray-200'>
            {data.map((item, index) => (
              <li
                key={item.id}
                className={`p-2 flex items-center justify-between cursor-pointer hover:bg-gray-100 ${selectedIndex === index ? 'bg-gray-100' : ''}`}
                onClick={() => handleItemClick(item)}
              >
                {item.verb}
                <span className='text-xs text-gray-500'>{item.template} {item.aspirate_h ? 'ℏ' : ''}</span>
              </li>
            ))}
          </ul>
        )}
        {!hasSearchResults && <EmptySearch />}
      </div>

      {
        selectedVerb && <Conjugation verb={selectedVerb} />
      }
    </div>
  )
}

function fetcher (url) {
  return fetch(url).then(res => res.json())
}
