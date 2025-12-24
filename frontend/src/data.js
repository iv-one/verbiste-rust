export const etre = [
  'aller',
  'venir',
  'arriver',
  'partir',
  'entrer',
  'sortir',
  'monter',
  'descendre',
  'naître',
  'mourir',
  'rester',
  'tomber',
  'retourner',
  'passer',
  'devenir',
  'revenir',
  'rentrer',
  'parvenir',
  'survenir',
  'intervenir',
  'advenir',
  'provenir',
  'convenir',
  'subvenir',
  'échoir',
  'éclore',
  'apparaître',
  'disparaître',
  'décéder',
  'ressusciter',
  'expirer'
]

export const avoir = [
  'entrer',
  'sortir',
  'monter',
  'descendre',
  'retourner',
  'passer',
  'rentrer',
  'éclore',
  'ressusciter',
  'expirer'
]

export const vander = [
  'aller',
  'venir',
  'arriver',
  'partir',
  'entrer',
  'sortir',
  'monter',
  'descendre',
  'naître',
  'mourir',
  'rester',
  'tomber',
  'retourner',
  'passer',
  'devenir',
  'revenir',
  'rentrer'
]

export const etreVerbs = (verb) => {
  return etre.includes(verb)
}

export const etreAndAvoirVerbs = (verb) => {
  return etre.includes(verb) && avoir.includes(verb)
}

export const emptyList = [
  { verb: 'être', template: ':être', aspirate_h: false },
  { verb: 'avoir', template: ':avoir', aspirate_h: false },
  { verb: 'aller', template: ':aller', aspirate_h: false },
  { verb: 'venir', template: 't:enir', aspirate_h: false },
  { verb: 'falloir', template: 'fa:lloir', aspirate_h: false },
  { verb: 'pouvoir', template: 'p:ouvoir', aspirate_h: false },
  { verb: 'vouloir', template: 'v:ouloir', aspirate_h: false },
  { verb: 'savoir', template: 's:avoir', aspirate_h: false },
  { verb: 'faire', template: 'f:aire', aspirate_h: false },
  { verb: 'prendre', template: 'pr:endre', aspirate_h: false },
  { verb: 'mettre', template: 'm:ettre', aspirate_h: false },
  { verb: 'dire', template: 'd:ire', aspirate_h: false },
  { verb: 'voir', template: 'v:oir', aspirate_h: false },
  { verb: 'boire', template: 'b:oire', aspirate_h: false },
  { verb: 'croire', template: 'cr:oire', aspirate_h: false },
  { verb: 'vivre', template: 'v:ivre', aspirate_h: false },
  { verb: 'partir', template: 'men:tir', aspirate_h: false },
  { verb: 'sortir', template: 'men:tir', aspirate_h: false },
  { verb: 'tenir', template: 't:enir', aspirate_h: false }
]
