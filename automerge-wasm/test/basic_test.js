
const assert = require('assert')
const util = require('util')
const Automerge = require('..')
const { MAP, LIST } = Automerge

describe('Automerge', () => {
  describe('basics', () => {
    it('should init clone and free', () => {
      let doc1 = Automerge.init()
      let doc2 = doc1.clone()
      doc1.free()
      doc2.free()
    })

    it('should be able to start and commit', () => {
      let doc = Automerge.init()
      doc.begin()
      doc.commit()
    })

    it('calling begin inside a transaction should throw an error', () => {
      let doc = Automerge.init()
      doc.begin()
      assert.throws(() => { doc.begin() }, Error);
    })

    it('calling commit outside a transaction should throw an error', () => {
      let doc = Automerge.init()
      assert.throws(() => { doc.commit() }, Error);
    })

    it('getting a nonexistant prop does not throw an error', () => {
      let doc = Automerge.init()
      let root = Automerge.root()
      let result = doc.value(root,"hello")
      assert.deepEqual(result,[])
    })

    it('should be able to set and get a simple value', () => {
      let doc = Automerge.init()
      let root = Automerge.root()
      let result

      doc.begin()
      doc.set(root, "hello", "world")
      doc.set(root, "number", 5, "uint")
      doc.commit()

      result = doc.value(root,"hello")
      assert.deepEqual(result,["str","world"])

      result = doc.value(root,"number")
      assert.deepEqual(result,["uint",5])
    })

    it('should be able to make sub objects', () => {
      let doc = Automerge.init()
      let root = Automerge.root()
      let result

      doc.begin()
      let submap = doc.set(root, "submap", MAP)
      doc.set(submap, "number", 6, "uint")
      assert.strictEqual(doc.pending_ops(),2)
      doc.commit()

      result = doc.value(root,"submap")
      assert.deepEqual(result,["map",submap])

      result = doc.value(submap,"number")
      assert.deepEqual(result,["uint",6])
    })

    it('should be able to make lists', () => {
      let doc = Automerge.init()
      let root = Automerge.root()

      doc.begin()
      let submap = doc.set(root, "numbers", LIST)
      doc.insert(submap, 0, "a");
      doc.insert(submap, 1, "b");
      doc.insert(submap, 2, "c");
      doc.insert(submap, 0, "z");
      doc.commit()
      assert.deepEqual(doc.value(submap, 0),["str","z"])
      assert.deepEqual(doc.value(submap, 1),["str","a"])
      assert.deepEqual(doc.value(submap, 2),["str","b"])
      assert.deepEqual(doc.value(submap, 3),["str","c"])
      assert.deepEqual(doc.length(submap),4)
    })

    it('should be able to del', () => {
      let doc = Automerge.init()
      let root = Automerge.root()

      doc.begin()
      doc.set(root, "xxx", "xxx");
      assert.deepEqual(doc.value(root, "xxx"),["str","xxx"])
      doc.del(root, "xxx");
      assert.deepEqual(doc.value(root, "xxx"),[])
      doc.commit()
    })
  })
})
