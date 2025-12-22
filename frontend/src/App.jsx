import './App.css'
import Search from './Search'
import { SWRConfig } from 'swr'

function App () {
  return (
    <SWRConfig value={{ provider: () => new Map() }}>
      <main className='antialiased p-4'>
        <Search />
      </main>
    </SWRConfig>
  )
}

export default App
