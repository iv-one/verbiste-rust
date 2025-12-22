export class Verb {
  constructor (verb, template) {
    this.verb = verb
    this.template = template

    const name = template.name
    const [, suffix] = name.split(':')
    this.base = verb.replace(suffix, '')
    this.suffix = suffix
  }
}
