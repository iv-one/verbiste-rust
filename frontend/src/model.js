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
