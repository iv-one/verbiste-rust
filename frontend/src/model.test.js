import { describe, it } from 'node:test'
import { expect } from 'chai'
import { Verb, getMaxWidth } from './model.js'

describe('Verb', function () {
  describe('#constructor', function () {
    it('should create a Verb with verb and template properties', function () {
      const template = { name: 'aim:er' }
      const verb = new Verb('aimer', template)
      expect(verb.verb).to.equal('aimer')
      expect(verb.template).to.equal(template)
    })

    it('should extract prefix and base from template name with prefix', function () {
      const template = { name: 'aim:er' }
      const verb = new Verb('aimer', template)
      expect(verb.suffix).to.equal('er')
      expect(verb.base).to.equal('aim')
    })

    it('should handle template name with colon at start', function () {
      const template = { name: ':être' }
      const verb = new Verb('être', template)
      expect(verb.suffix).to.equal('être')
      expect(verb.base).to.equal('')
    })
  })
})

describe('getMaxWidth', function () {
  it('should return the max width of a template', function () {
    const template = {
      name: 'aim:er',
      infinitive: {
        infinitive_present: ['er']
      }
    }
    expect(getMaxWidth(template)).to.equal(1)
  })

  it('should return the max width of a template with nested array', function () {
    const template = {
      name: 'aim:er',
      cond: ['urre'],
      infinitive: {
        infinitive_present: ['er', 'mer']
      }
    }
    expect(getMaxWidth(template)).to.equal(2)
  })
})
