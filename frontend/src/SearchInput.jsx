import { useEffect, useRef } from 'react'
import { X } from 'lucide-react'

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

  const handleClear = () => {
    const syntheticEvent = {
      target: { value: '' }
    }
    onChange(syntheticEvent)
    if (inputRef.current) {
      inputRef.current.focus()
    }
  }

  const hasValue = value && value.trim() !== ''

  return (
    <div className='relative'>
      <input
        ref={inputRef}
        type='text'
        placeholder={placeholder}
        className='search-input w-full p-2 pr-10 border-0 bg-transparent focus:outline-none'
        value={value || ''}
        onChange={onChange}
        onKeyDown={handleKeyDown}
      />
      {hasValue && (
        <button
          type='button'
          onClick={handleClear}
          className='absolute border-2 right-2 top-1/2 -translate-y-1/2 w-6 h-6 rounded-full flex items-center justify-center hover:bg-gray-200 transition-colors'
          aria-label='Clear search'
        >
          <X size={14} className='text-gray-500' />
        </button>
      )}
    </div>
  )
}
