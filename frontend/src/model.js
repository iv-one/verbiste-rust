export const Faces = [
  'je',
  'tu',
  'il / elle / on',
  'ils / elles',
  'nous',
  'vous'
]

export class Verb {
  constructor (verb, template) {
    this.verb = verb
    this.template = template

    const name = template.name
    const [, suffix] = name.split(':')
    this.base = verb.replace(suffix, '')
    this.suffix = suffix
    this.maxWidth = getMaxWidth(template)
  }

  derive (field) {
    const path = field.split('.')
    let value = this.template
    for (const p of path) {
      value = value[p]
    }
    return deriveVerbs(this.base, value)
  }

  get infinitive () {
    return this.derive('infinitive.infinitive_present')
  }
}

// iterate over template fields, if field is an array, get the max length
// if it's an object, call getMaxWidth on the object
export const getMaxWidth = (template) => {
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
      return suffix.map(s => `${base}${s}`)
    } else {
      return `${base}${suffix}`
    }
  })
  return reorderVerbs(res)
}

export const reorderVerbs = (verbs) => {
  // take the latest element and insert in position 4
  const latest = verbs.pop()
  verbs.splice(3, 0, latest)
  return verbs
}
