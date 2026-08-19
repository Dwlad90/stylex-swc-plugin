# 07 — Delete the imported-name fallback

Status: `ready-for-agent`
Blocked by: 04

**What to build:** An import stops resolving through the name it was aliased
*away from*.

The import lookup has a second half with no counterpart in the reference
implementation: after failing to match a reference against a specifier's local
binding, it tries the specifier's *imported* name. So a reference to `zIndex`
could resolve to `import { zIndex as zi }` — a binding that does not exist under
that name in any scope.

One arm of it is unreachable by construction: the imported identifier carries
the unresolved syntax context and never matches a reference's. The other arm,
for string-named specifiers, matches on symbol alone and remains live.

It is also the only route to a latent abort. Had the branch matched, the caller
re-searches by *local* name, finds nothing, and aborts with `Could not resolve
the import specifier` — a panic reachable through this branch and nowhere else.
Deleting the branch deletes the panic's only route.

Delete both arms and the now-unreachable abort. The suite is the check: if
something depended on resolving an import by its aliased-away name, that is the
finding, and it belongs in this ticket's comments rather than being worked
around.

- [ ] Both arms of the fallback are gone
- [ ] The abort reachable only through them is gone
- [ ] Full suite green, or the dependency it exposed is recorded
- [ ] A corpus guard: a reference whose name matches an import's aliased-away
      name resolves the way the reference implementation resolves it
