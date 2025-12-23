import useSWR from 'swr'
import { useEffect } from 'react'
import { Verb, Faces } from './model'
import { etreVerbs, etreAndAvoirVerbs } from './data'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { take } from 'lodash'
import { useQueryState } from 'nuqs'

export default function Conjugation ({ verb }) {
  const [useEtre, setUseEtre] = useQueryState('ax')

  const { data } = useSWR(`/api/t/${verb.template}`, fetcher)
  const { data: etreData } = useSWR('/api/t/:être', fetcher)
  const { data: avoirData } = useSWR('/api/t/:avoir', fetcher)

  const word = new Verb(verb.verb, data)
  const etre = new Verb('être', etreData)
  const avoir = new Verb('avoir', avoirData)

  const rows = [0, 1, 2, 3, 4, 5]

  const isEtre = etreVerbs(verb.verb)
  const isEtreAndAvoir = etreAndAvoirVerbs(verb.verb)

  useEffect(() => {
    setUseEtre(isEtre ? 'etre' : 'avoir')
  }, [data])

  const ax = useEtre === 'etre' ? etre : avoir

  if (!data || !etreData || !avoirData) {
    return <div>Loading...</div>
  }

  return (
    <div className='p-2'>
      <div className='flex items-center gap-8'>
        <div className='flex items-center divide-x divide-gray-200'>
          <div className='pr-2 flex items-center'>
            {renderCell(word.infinitive)}
            <div className='mx-2 text-xs text-gray-500 bg-gray-100 font-semibold rounded-md p-1 px-1.5'>{word.group}</div>
          </div>
          <div className='px-2 text-sm text-gray-500'>{verb.template}</div>
          <div className='px-2'>{renderCell(word.presentParticiple)}</div>
          <div className='px-2'>{word.participle}</div>
        </div>
        <div>
          <AxTabs etre={isEtre} avoir={isEtreAndAvoir} />
        </div>
      </div>

      <table className='conjugation-table mt-8'>
        <thead>
          <th />
          <th>Passé simple</th>
          <th>Imparfait</th>
          <th>Présent</th>
          <th>Conditionnel</th>
          <th>Futur</th>
        </thead>
        <tbody>
          {rows.map(n => renderRow(n, word))}
        </tbody>
        <thead className='secondary'>
          <th />
          <th />
          <th>Subjonctif imparfait</th>
          <th>Subjonctif</th>
          <th />
          <th />
        </thead>
        <tbody>
          {rows.map(n => renderRowSub(n, word))}
        </tbody>
        <thead className='secondary'>
          <th />
          <th>Passé antérieur</th>
          <th>Plus-que-parfait</th>
          <th>Passé composé</th>
          <th>Conditionnel passé</th>
          <th>Futur antérieur</th>
        </thead>
        <tbody>
          {rows.map(n => renderRowComposite(n, word, ax))}
        </tbody>
        <thead className='secondary'>
          <th />
          <th>Participe passé</th>
          <th>Imperatif présent</th>
          <th />
          <th />
          <th />
        </thead>
        <tbody>
          {take(rows, 4).map(n => renderLastRow(n, word))}
        </tbody>
      </table>
    </div>
  )
}

function renderRow (n, word) {
  return (
    <tr key={n}>
      <td>{Faces[n]}</td>
      <td>{renderCell(word.simplePast[n])}</td>
      <td>{renderCell(word.imperfect[n])}</td>
      <td>{renderCell(word.present[n])}</td>
      <td>{renderCell(word.conditional[n])}</td>
      <td>{renderCell(word.future[n])}</td>
    </tr>
  )
}

function renderRowSub (n, word) {
  return (
    <tr key={n}>
      <td>{Faces[n]}</td>
      <td />
      <td>{renderCell(word.subjunctiveImperfect[n])}</td>
      <td>{renderCell(word.subjunctive[n])}</td>
      <td />
      <td />
    </tr>
  )
}

function renderRowComposite (n, word, ax) {
  const participle = word.participle
  const present = word.present[n]
  return (
    <tr key={n}>
      <td>{Faces[n]}</td>
      <td>{renderCellComposite(ax.simplePast[n], participle, present)}</td>
      <td>{renderCellComposite(ax.imperfect[n], participle, present)}</td>
      <td>{renderCellComposite(ax.present[n], participle, present)}</td>
      <td>{renderCellComposite(ax.conditional[n], participle, present)}</td>
      <td>{renderCellComposite(ax.future[n], participle, present)}</td>
    </tr>
  )
}

function renderLastRow (n, word) {
  return (
    <tr key={n}>
      <td />
      <td>{renderCell(word.pastParticiple[n])}</td>
      <td>{renderCell(word.imperativePresent[n])}</td>
      <td />
      <td />
      <td />
    </tr>
  )
}

function renderCell (cell) {
  if (!cell) {
    return ''
  }
  return cell.join(' / ').trim()
}

function renderCellComposite (cell, participle, present) {
  const hasPresent = present.filter(Boolean).length > 0
  if (!cell || !hasPresent) {
    return ''
  }
  return `${cell.join(' / ').trim()} ${participle}`
}

function fetcher (url) {
  return fetch(url).then(res => res.json())
}

const AxTabs = ({ etre, avoir }) => {
  const [useEtre, setUseEtre] = useQueryState('ax')

  const showEtre = etre
  const showAvoir = !etre || avoir
  return (
    <Tabs value={useEtre} onValueChange={setUseEtre}>
      <TabsList>
        {showEtre && <TabsTrigger value='etre'>Être</TabsTrigger>}
        {showAvoir && <TabsTrigger value='avoir'>Avoir</TabsTrigger>}
      </TabsList>
    </Tabs>
  )
}
