# o3-dapi-core
Core messaging protocol.

Accepts plugins for different blockchains.

A plugin is required for use.

## Usage

In a browser - cdn [![](https://data.jsdelivr.com/v1/package/npm/o3-dapi-core/badge)](https://www.jsdelivr.com/package/npm/o3-dapi-core)
```
<script src="https://cdn.jsdelivr.net/npm/o3-dapi-core/lib/o3-dapi-core.min.js"></script>
```
```
window.o3dapi
```

Install via npm [![npm version](https://badge.fury.io/js/o3-dapi-core.svg)](https://badge.fury.io/js/o3-dapi-core)
```
npm i --save o3-dapi-core

or

yarn add o3-dapi-core
```

```
var o3dapi = require('o3-dapi-core');

import o3dapi from 'o3-dapi-core';
```

## Example
```
o3dapi.initPlugins([<neo_plugin>, <ont_plugin>]);
```

## React Native

For use with React Native, please pass in the ReactNative module into the `setReactNativeOverrides` method.

This will broadcast any dapi messages from the client to the `DapiBridge.handleMessage` method in your native mobile app.

The client app will also listen to the `o3dapiEvent` event from the DeviceEventEmitter. Events should be passed with the format:

```
interface Event {
  eventName: string;
  data: any;
}
```

Initialization on from the react native app should look like this:

```
import o3dapi from 'o3-dapi-core';
import ReactNative from 'react-native';

o3dapi.setReactNativeOverrides(ReactNative);
```
