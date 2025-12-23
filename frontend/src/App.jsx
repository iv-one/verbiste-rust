import './App.css'
import Search from './Search'
import { SWRConfig } from 'swr'
import { NuqsAdapter } from 'nuqs/adapters/react'

function App () {
  return (
    <NuqsAdapter>
      <SWRConfig value={{ provider: () => new Map() }}>
        <main className='antialiased p-4'>
          <Search />
        </main>
      </SWRConfig>
    </NuqsAdapter>
  )
}

export default App
