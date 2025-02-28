# neo3-dapi
A standalone bundle for interfacing with the NEO N3 blockchain

## Installation

In a browser - cdn [![](https://data.jsdelivr.com/v1/package/npm/neo3-dapi/badge)](https://www.jsdelivr.com/package/npm/neo3-dapi)
```
<script src="https://cdn.jsdelivr.net/npm/neo3-dapi@1.0.1/lib/neo3-dapi.min.js"></script>
```
```
window.neo3Dapi
```

Install via npm [![npm version](https://badge.fury.io/js/neo3-dapi.svg)](https://badge.fury.io/js/neo3-dapi)
```
npm i --save neo3-dapi

or

yarn add neo3-dapi
```

```
var neo3Dapi = require('neo3-dapi');

import neo3Dapi from 'neo3-dapi';
```

## Usage Example

The following is an example of requesting the balances for a specific address.
```
import neo3Dapi from 'neo3-dapi';

neo3Dapi.getBalance([{
  address: 'NfuwpaQ1A2xaeVbxWe8FRtaRgaMa8yF3YM',
  contracts: ['NEO'],
}])
.then(balances => console.log(balances));
```

## Documentation



