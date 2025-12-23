import { useQueryState } from 'nuqs'
import { useEffect, useRef } from 'react'
import useSWR from 'swr'
import Conjugation from './Conjugation'
import { EmptySearch } from './Empty'

export default function Search () {
  const [search, setSearch] = useQueryState('q')
  const inputRef = useRef(null)
  const { data } = useSWR(`/api/search?q=${search}`, fetcher)
  const hasData = data && search && data.length > 1
  const hasSearchResults = data && search && data.length > 0

  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.focus()
    }
  }, [])

  return (
    <div>
      <h1 className='text-xs border-b border-gray-200 pb-1 w-md px-2 text-gray-500'>Verb</h1>
      <input
        ref={inputRef}
        type='text'
        placeholder='Search'
        className='search-input w-full p-2 border-0 bg-transparent focus:outline-none'
        value={search || ''}
        onChange={(e) => setSearch(e.target.value)}
      />

      <div className='mt-2 max-w-md'>
        {hasData && (
          <ul className='divide-y divide-gray-200'>
            {data.map(item => (
              <li className='p-2 flex items-center justify-between cursor-pointer hover:bg-gray-100' key={item.id} onClick={() => setSearch(item.verb)}>
                {item.verb}
                <span className='text-xs text-gray-500'>{item.template} {item.aspirate_h ? 'ℏ' : ''}</span>
              </li>
            ))}
          </ul>
        )}
        {!hasSearchResults && <EmptySearch />}
      </div>

      {
        data && data.length === 1 && <Conjugation verb={data[0]} />
      }
    </div>
  )
}

function fetcher (url) {
  return fetch(url).then(res => res.json())
}
