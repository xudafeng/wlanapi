'use strict';

const assert = require('assert');

const wlanapi = require('../lib');

describe('test', () => {
  it('should be ok', (done) => {
    wlanapi.registerNotification((data) => {
      console.log(data);
      console.log('hi wifi changed');
    });
    setTimeout(() => { done() }, 10 * 1000);
  });
});
