export const Faces = [
  'je',
  'tu',
  'il / elle / on',
  'ils / elles',
  'nous',
  'vous'
]

export class Verb {
  constructor (verb, template = {}) {
    this.verb = verb
    this.template = template
    this.cache = new Map()

    const name = template.name || ''
    const [prefix, suffix] = name.split(':')

    this.prefix = prefix
    this.base = verb.replace(suffix, '')
    this.suffix = suffix
    this.maxWidth = Math.max(this.infinitive.length, this.future[0].length)

    this.group = detectGroup(verb, suffix, this.participle)
  }

  derive (field) {
    if (this.cache.has(field)) {
      return this.cache.get(field)
    }

    const path = field.split('.')
    let value = this.template
    for (const p of path) {
      value = value[p]
    }
    const result = deriveVerbs(this.base, value)
    this.cache.set(field, result)
    return result
  }

  get infinitive () {
    return this.derive('infinitive.infinitive_present')
  }

  // participe présent
  get presentParticiple () {
    return this.derive('participle.present_participle')
  }

  // participe passé
  get pastParticiple () {
    return this.derive('participle.past_participle')
  }

  // participe passé main form
  get participle () {
    return this.pastParticiple[0][0]
  }

  get simplePast () {
    return this.derive('indicative.simple_past')
  }

  get imperfect () {
    return this.derive('indicative.imperfect')
  }

  get present () {
    return this.derive('indicative.present')
  }

  get future () {
    return this.derive('indicative.future')
  }

  get conditional () {
    return this.derive('conditional.present')
  }

  get subjunctive () {
    return this.derive('subjunctive.present')
  }

  get subjunctiveImperfect () {
    return this.derive('subjunctive.imperfect')
  }

  get imperativePresent () {
    return this.derive('imperative.imperative_present')
  }
}

// iterate over template fields, if field is an array, get the max length
// if it's an object, call getMaxWidth on the object
export const getMaxWidth = (template = {}) => {
  let maxWidth = 0
  for (const field of Object.values(template)) {
    if (Array.isArray(field)) {
      maxWidth = Math.max(maxWidth, field.length)
    } else if (typeof field === 'object') {
      maxWidth = Math.max(maxWidth, getMaxWidth(field))
    }
  }
  return maxWidth
}

// suffixes is an array<array<string>>
// deriveVerbs(string, array<array<string>>) -> array<string>
export const deriveVerbs = (base, suffixes) => {
  const res = suffixes.map(suffix => {
    if (Array.isArray(suffix)) {
      return suffix.map(s => s ? `${base}${s}` : '')
    } else {
      return suffix ? `${base}${suffix}` : ''
    }
  })
  return reorderVerbs(res)
}

export const reorderVerbs = (verbs) => {
  if (verbs.length < 6) {
    return verbs
  }
  // take the latest element and insert in position 4
  const latest = verbs.pop()
  verbs.splice(3, 0, latest)
  return verbs
}

export const detectGroup = (verb, suffix, participle) => {
  if (suffix === 'er' && verb !== 'aller') {
    return 1
  }
  if (suffix === 'ir' && participle.includes('ssant')) {
    return 2
  }
  return 3
}
