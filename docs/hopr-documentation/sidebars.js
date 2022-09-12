/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  tutorialSidebar: [{ type: 'autogenerated', dirName: '.' }],

  // But you can create a sidebar manually

  tutorialSidebar: [
    {
      type: 'category',
      label: 'What is HOPR?',
      items: [
        'core/what-is-hopr',
        'core/what-is-metadata',
        'core/anonymous-routing',
        'core/mixnets',
        'core/incentives',
        'core/proof-of-relay',
        'core/tickets-and-payment-channels',
        'core/probabilistic-payments',
        'core/cover-traffic',
        'core/cover-traffic-nodes',
        'core/balancing-cover-traffic',
        'faq'
      ]
    },

    {
      type: 'category',
      label: 'Run a HOPR node',
      items: ['node/start-here',
        'node/using-avado',
        'node/using-docker',
        'node/using-hopr-admin',
        'node/hoprd-commands']
    },

    {
      type: 'category',
      label: 'Developers',
      link: { type: 'doc', id: 'developers/intro' },
      items: [
        'developers/starting-local-cluster',
        'developers/connecting-node',
        'developers/network-registry',
        'developers/tutorial-hello-world',
        'developers/demo-boomerang-chat',
        'developers/demo-rps-game',
        'developers/rest-api',
        'developers/smart-contract',
        'developers/snippets',
        'developers/visualising-hopr-network-topology'
      ]
    },

    {
      type: 'category',
      label: 'HOPR dApps',
      items: ['dapps/myne-chat']
    },

    {
      type: 'category',
      label: 'Ecosystem',
      items: [
        'about-hopr',
        'ecosystem/hopr-token',
        'ecosystem/gnosis-hopr-tokens',
        'ecosystem/staking',
        'ecosystem/hoprd',
        'staking/how-to-get-hopr',
        'staking/how-to-stake',
        'staking/convert-hopr'
      ]
    }

    /*
    {
      type: 'category',
      label: 'About HOPR',
      items: ['about-hopr']
    },
    {
      type: 'category',
      label: 'Installing a hoprd node',
      items: ['node/start-here', 'node/using-avado', 'node/using-docker']
    },
    {
      type: 'category',
      label: 'Run a hoprd node',
      items: ['node/guide-using-a-hoprd-node', 'node/hoprd-commands']
    },
    {
      type: 'category',
      label: 'HOPR core concepts',
      items: [
        'core/what-is-hopr',
        'core/what-is-metadata',
        'core/anonymous-routing',
        'core/mixnets',
        'core/incentives',
        'core/proof-of-relay',
        'core/tickets-and-payment-channels',
        'core/probabilistic-payments',
        'core/cover-traffic',
        'core/cover-traffic-nodes',
        'core/balancing-cover-traffic'
      ]
    },
    {
      type: 'category',
      label: 'Staking',
      items: ['staking/how-to-get-hopr', 'staking/how-to-stake', 'staking/convert-hopr']
    },
    {
      type: 'category',
      label: 'Developers',
      link: { type: 'doc', id: 'developers/intro' },
      items: [
        'developers/starting-local-cluster',
        'developers/connecting-node',
        'developers/network-registry',
        'developers/tutorial-hello-world',
        'developers/demo-boomerang-chat',
        'developers/demo-rps-game',
        'developers/rest-api',
        'developers/smart-contract',
        'developers/snippets',
        'developers/visualising-hopr-network-topology'
      ]
    },
    {
      type: 'category',
      label: 'dApps',
      items: ['dapps/myne-chat']
    },
    {
      type: 'category',
      label: 'Ecosystem',
      link: { type: 'doc', id: 'ecosystem/introduction' },
      items: ['ecosystem/hopr-token', 'ecosystem/gnosis-hopr-tokens', 'ecosystem/staking', 'ecosystem/hoprd']
    },
    {
      type: 'doc',
      id: 'faq',
      label: 'FAQ'
    }*/
  ]
}

module.exports = sidebars
