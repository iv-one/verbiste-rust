import { useEffect, useRef } from 'react'

export default function SearchInput ({ value, onChange, onKeyDown, placeholder = 'Search' }) {
  const inputRef = useRef(null)

  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.focus()
    }
  }, [])

  const handleKeyDown = (e) => {
    if (e.key === 'Escape') {
      const syntheticEvent = {
        target: { value: '' }
      }
      onChange(syntheticEvent)
    } else if (onKeyDown && (e.key === 'ArrowDown' || e.key === 'ArrowUp' || e.key === 'Enter')) {
      onKeyDown(e)
    }
  }

  return (
    <input
      ref={inputRef}
      type='text'
      placeholder={placeholder}
      className='search-input w-full p-2 border-0 bg-transparent focus:outline-none'
      value={value || ''}
      onChange={onChange}
      onKeyDown={handleKeyDown}
    />
  )
}
