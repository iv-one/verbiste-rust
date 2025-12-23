import { IconFolderCode } from '@tabler/icons-react'
import { ArrowUpRightIcon } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'

export function EmptySearch () {
  return (
    <Empty>
      <EmptyHeader>
        <EmptyMedia variant='icon'>
          <IconFolderCode />
        </EmptyMedia>
        <EmptyTitle>No Search Results</EmptyTitle>
        <EmptyDescription>
          No verbs found! Maybe you conjured up a new word? Try searching for another verb and unleash your inner French wizard
        </EmptyDescription>
      </EmptyHeader>
      <Button
        variant='link'
        asChild
        className='text-muted-foreground'
        size='sm'
      >
        <a href='#'>
          Learn More <ArrowUpRightIcon />
        </a>
      </Button>
    </Empty>
  )
}
