# Release Checklist

## Pre-release

- [ ] All tests passing (`./test-all.sh`)
- [ ] CI pipeline green
- [ ] Documentation up to date
- [ ] CHANGELOG.md updated
- [ ] Version bumped in package.json
- [ ] Examples work correctly

## Publishing to npm

```bash
npm login
npm publish
```

## Creating GitHub Release

1. Tag the release:
   ```bash
   git tag -a tree-sitter-neon-v0.1.0 -m "Release tree-sitter-neon v0.1.0"
   git push origin tree-sitter-neon-v0.1.0
   ```

2. Create release on GitHub with changelog

## Submitting to nvim-treesitter

1. Fork https://github.com/nvim-treesitter/nvim-treesitter
2. Add parser configuration
3. Submit PR

## Announcing

- [ ] Post on Neon repository
- [ ] Update documentation
- [ ] Share with community
