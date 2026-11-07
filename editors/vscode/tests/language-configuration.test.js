'use strict';

const test = require('node:test');
const assert = require('node:assert');
const fs = require('fs');
const path = require('path');

const { onEnterRules } = JSON.parse(
  fs.readFileSync(path.join(__dirname, '..', 'language-configuration.json'), 'utf8')
);

function onEnter(beforeText, afterText = '') {
  return onEnterRules.find(
    (r) =>
      new RegExp(r.beforeText).test(beforeText) &&
      (!r.afterText || new RegExp(r.afterText).test(afterText))
  );
}

test('continues a whole-line comment on Enter', () => {
  for (const before of ['    // note', '// no', '\t// note']) {
    const rule = onEnter(before);
    assert.ok(rule, before);
    assert.deepStrictEqual(rule.action, { indent: 'none', appendText: '// ' });
  }
});

test('does not continue a trailing comment on Enter', () => {
  assert.strictEqual(onEnter('val x = 1 // note'), undefined);
});

test('does not continue outside a comment on Enter', () => {
  for (const before of ['', '    ', '    /']) {
    assert.strictEqual(onEnter(before), undefined, JSON.stringify(before));
  }
});
