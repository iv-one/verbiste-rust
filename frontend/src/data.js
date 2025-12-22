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
