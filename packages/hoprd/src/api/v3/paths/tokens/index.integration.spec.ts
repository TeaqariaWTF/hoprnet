import request from 'supertest'
import sinon from 'sinon'
import chaiResponseValidator from 'chai-openapi-response-validator'
import chai, { expect } from 'chai'

import { createAuthenticatedTestApiInstance } from './../../fixtures.js'
import { STATUS_CODES } from './../../utils.js'

import type { default as Hopr } from '@hoprnet/hopr-core'
import { LevelDb } from '@hoprnet/hopr-utils'
import { Address } from '../../../../../lib/hoprd_misc.js'

import { Database } from '../../../../../../core/lib/core_hopr.js'

describe('POST /tokens', function () {
  let node: Hopr
  let service: any

  before(async function () {
    node = sinon.fake() as any
    let db = new LevelDb()
    await db.backend.open()
    node.db = new Database(db, Address.from_string('0xf55df5f3ce0ccce707f76ef3e8459adff316ac99'))

    const loaded = await createAuthenticatedTestApiInstance(node)

    service = loaded.service

    // @ts-ignore ESM / CommonJS compatibility issue
    chai.use(chaiResponseValidator.default(loaded.api.apiDoc))
  })

  it('should fail with parameter error when using superuser token and incorrect parameters', async function () {
    let res

    const parametersWrongDescription = {
      description: 123,
      capabilities: [{ endpoint: 'tokensCreate' }]
    }
    res = await request(service)
      .post('/api/v3/tokens')
      .set('x-auth-token', 'superuser')
      .send(parametersWrongDescription)
    expect(res.status).to.equal(400)
    expect(res.body.status).to.equal(STATUS_CODES.INVALID_INPUT)
    expect(res).to.satisfyApiSpec

    const parametersWrongLifetime = {
      lifetime: 0,
      capabilities: [{ endpoint: 'tokensCreate' }]
    }
    res = await request(service).post('/api/v3/tokens').set('x-auth-token', 'superuser').send(parametersWrongLifetime)
    expect(res.status).to.equal(400)
    expect(res.body.status).to.equal(STATUS_CODES.INVALID_INPUT)
    expect(res).to.satisfyApiSpec

    const parametersWrongCapabilitiesMissing = {
      lifetime: 0
    }
    res = await request(service)
      .post('/api/v3/tokens')
      .set('x-auth-token', 'superuser')
      .send(parametersWrongCapabilitiesMissing)
    expect(res.status).to.equal(400)
    expect(res.body.status).to.equal(STATUS_CODES.INVALID_INPUT)
    expect(res).to.satisfyApiSpec

    const parametersWrongCapabilitiesEmpty = {
      capabilities: []
    }
    res = await request(service)
      .post('/api/v3/tokens')
      .set('x-auth-token', 'superuser')
      .send(parametersWrongCapabilitiesEmpty)
    expect(res.status).to.equal(400)
    expect(res.body.status).to.equal(STATUS_CODES.INVALID_INPUT)
    expect(res).to.satisfyApiSpec

    const parametersWrongCapabilitiesEndpointMissing = {
      capabilities: [{ limits: [{ type: 'calls', conditions: { max: 1 } }] }]
    }
    res = await request(service)
      .post('/api/v3/tokens')
      .set('x-auth-token', 'superuser')
      .send(parametersWrongCapabilitiesEndpointMissing)
    expect(res.status).to.equal(400)
    expect(res.body.status).to.equal(STATUS_CODES.INVALID_INPUT)
    expect(res).to.satisfyApiSpec
  })

  it('should succeed when using superuser token and correct parameters', async function () {
    let res

    const parametersOnlyCapabilities = {
      capabilities: [{ endpoint: 'tokensCreate' }]
    }
    res = await request(service)
      .post('/api/v3/tokens')
      .set('x-auth-token', 'superuser')
      .send(parametersOnlyCapabilities)
    expect(res.status).to.equal(201)
    expect(res.body.token).to.not.be.empty
    expect(res).to.satisfyApiSpec

    const parametersFull = {
      lifetime: 1,
      description: 'todo',
      capabilities: [{ endpoint: 'tokensCreate' }]
    }
    res = await request(service).post('/api/v3/tokens').set('x-auth-token', 'superuser').send(parametersFull)
    expect(res.status).to.equal(201)
    expect(res.body.token).to.not.be.empty
    expect(res).to.satisfyApiSpec
  })
})
