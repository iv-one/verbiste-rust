import { describe, it } from 'node:test'
import { expect } from 'chai'
import { Verb, getMaxWidth, deriveVerbs } from './model.js'
import { aimer } from './testdata/index.js'

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

describe('deriveVerbs', function () {
  it('should derive verbs from a base and suffixes', function () {
    const present = [['e'], ['es'], ['e'], ['ons'], ['ez'], ['ent']]
    const expected = [['aime'], ['aimes'], ['aime'], ['aiment'], ['aimons'], ['aimez']]
    expect(deriveVerbs('aim', present)).to.deep.equal(expected)
  })

  it('should derive verbs from a base and suffixes II', function () {
    const present = ['er']
    const expected = ['aimer']
    expect(deriveVerbs('aim', present)).to.deep.equal(expected)
  })
})

describe('Verb aimer', function () {
  const verb = new Verb('aimer', aimer)
  it('infinitive present', function () {
    expect(verb.infinitive).to.deep.equal(['aimer'])
  })

  // simplePast
  it('simple past', function () {
    expect(verb.simplePast).to.deep.equal([['aimai'], ['aimas'], ['aima'], ['aimèrent'], ['aimâmes'], ['aimâtes']])
  })

  // imparfait
  it('imperfect', function () {
    expect(verb.imperfect).to.deep.equal([['aimais'], ['aimais'], ['aimait'], ['aimaient'], ['aimions'], ['aimiez']])
  })

  // conditionnel
  it('conditional', function () {
    expect(verb.conditional).to.deep.equal([['aimerais'], ['aimerais'], ['aimerait'], ['aimeraient'], ['aimerions'], ['aimeriez']])
  })

  // future
  it('future', function () {
    expect(verb.future).to.deep.equal([['aimerai'], ['aimeras'], ['aimera'], ['aimeront'], ['aimerons'], ['aimerez']])
  })

  // subjunctive
  it('subjunctive', function () {
    expect(verb.subjunctive).to.deep.equal([['aime'], ['aimes'], ['aime'], ['aiment'], ['aimions'], ['aimiez']])
  })
})
