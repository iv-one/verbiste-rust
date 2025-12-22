import { useState } from 'react'
import useSWR from 'swr'

export default function Search () {
  const [search, setSearch] = useState('')
  const { data } = useSWR(`/api/search?q=${search}`, fetcher)
  return (
    <div>
      <h1 className='text-xs border-b border-gray-200 pb-1 w-md px-2 text-gray-500'>Verbiste</h1>
      <input type='text' placeholder='Search' className='w-full p-2 border-0 bg-transparent focus:outline-none' value={search} onChange={(e) => setSearch(e.target.value)} />

      <div className='mt-2 max-w-md'>
        {
        data && (
          <ul className='divide-y divide-gray-200'>
            {data.map(item => (
              <li className='p-2 flex items-center justify-between cursor-pointer hover:bg-gray-100' key={item.id} onClick={() => setSearch(item.verb)}>
                {item.verb}
                <span className='text-xs text-gray-500'>{item.template} {item.aspirate_h ? 'ℏ' : ''}</span>
              </li>
            ))}
          </ul>
        )
      }
      </div>
    </div>
  )
}

function fetcher (url) {
  return fetch(url).then(res => res.json())
}
