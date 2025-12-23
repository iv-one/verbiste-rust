import useSWR from 'swr'
import { Verb, Faces } from './model'
import { etreVerbs, etreAndAvoirVerbs } from './data'

export default function Conjugation ({ verb }) {
  const { data } = useSWR(`/api/t/${verb.template}`, fetcher)
  const { data: etreData } = useSWR('/api/t/:être', fetcher)
  const { data: avoirData } = useSWR('/api/t/:avoir', fetcher)

  if (!data || !etreData || !avoirData) {
    return <div>Loading...</div>
  }

  const word = new Verb(verb.verb, data)
  const etre = new Verb('être', etreData)
  const avoir = new Verb('avoir', avoirData)

  const rows = [0, 1, 2, 3, 4, 5]

  const isEtre = etreVerbs(verb.verb)
  const isEtreAndAvoir = etreAndAvoirVerbs(verb.verb)

  const ax = isEtre ? etre : avoir

  return (
    <div className='p-2'>
      <div className='flex items-center divide-x divide-gray-200'>
        <div className='pr-2 flex items-center'>
          {renderCell(word.infinitive)}
          <div className='mx-2 text-xs text-gray-500 bg-gray-100 font-semibold rounded-md p-1 px-1.5'>{word.group}</div>
        </div>
        <div className='px-2 text-sm text-gray-500'>{verb.template}</div>
        <div className='px-2'>{renderCell(word.presentParticiple)}</div>
        <div className='px-2'>{word.participle}</div>
      </div>
      <div>{verb.verb}</div>
      <div>{verb.template}</div>
      <div>{word.participle}</div>
      <div>participe présent: {renderCell(word.presentParticiple)}</div>
      <div>infinitive: {renderCell(word.infinitive)}</div>
      <div>max width: {word.maxWidth}</div>
      <div>is etre: {isEtre ? 'true' : 'false'}</div>
      <div>is etre and avoir: {isEtreAndAvoir ? 'true' : 'false'}</div>
      <table className='conjugation-table'>
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
          {rows.map(n => renderRowComposite(n, ax, word.participle))}
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

function renderRowComposite (n, word, participle) {
  return (
    <tr key={n}>
      <td>{Faces[n]}</td>
      <td>{renderCell(word.simplePast[n])} {participle}</td>
      <td>{renderCell(word.imperfect[n])} {participle}</td>
      <td>{renderCell(word.present[n])} {participle}</td>
      <td>{renderCell(word.conditional[n])} {participle}</td>
      <td>{renderCell(word.future[n])} {participle}</td>
    </tr>
  )
}

function renderCell (cell) {
  if (!cell) {
    return ' - '
  }
  return cell.join(' / ').trim()
}

function fetcher (url) {
  return fetch(url).then(res => res.json())
}
