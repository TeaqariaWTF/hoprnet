import Hopr from '..'
import { HoprCoreConnectorInstance } from '@hoprnet/hopr-core-connector-interface'

import PeerId from 'peer-id'
import PeerInfo from 'peer-info'
import Multiaddr from 'multiaddr'


export type Sink = (source: AsyncIterable<Uint8Array>) => void

export type Source = AsyncIterator<Uint8Array>

export type Duplex = {
  sink: Sink,
  source: Source
}

abstract class AbstractInteraction<Chain extends HoprCoreConnectorInstance> {
  protected protocols: string[]
  protected node: Hopr<Chain>

  constructor(node: Hopr<Chain>, protocols: string[]) {
    this.protocols = protocols
    this.node = node

    node.handle(this.protocols, this.handler.bind(this))
  }

  abstract handler(struct: { stream: Duplex }): void

  abstract interact(counterparty: PeerInfo | PeerId | Multiaddr, ...props: any[]): any
}

export { AbstractInteraction }
