import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const [clientPackage, mode, revision, caseFile] = process.argv.slice(2);
const { server, call, record } = JSON.parse(readFileSync(caseFile, "utf8"));
const modernSdk = clientPackage === "@modelcontextprotocol/client" || mode === "auto";
const { Client } = await import(modernSdk ? clientPackage : `${clientPackage}/client/index.js`);
const { StdioClientTransport } = await import(
  modernSdk ? `${clientPackage}/stdio` : `${clientPackage}/client/stdio.js`
);
const client = new Client(
  { name: "biomcp-interop", version: "1.0.0" },
  { capabilities: {}, versionNegotiation: { mode } },
);
const transport = new StdioClientTransport(server);
let legacyRevision;
if (!modernSdk) {
  // The v1 clients validate initialize but expose no negotiated revision accessor.
  const start = transport.start.bind(transport);
  transport.start = () => {
    const receive = transport.onmessage;
    transport.onmessage = (message) => {
      legacyRevision ??= message.result?.protocolVersion;
      receive(message);
    };
    return start();
  };
}

let step = "negotiation";
try {
  await client.connect(transport);
  assert.equal(modernSdk ? client.getNegotiatedProtocolVersion() : legacyRevision, revision);
  step = "tools/list";
  const listing = await client.listTools();
  assert(listing.tools.some(({ name }) => name === call.name), "biomcp tool missing");
  step = "tools/call";
  const result = await client.callTool(call);
  assert(!result.isError, "biomcp reported a tool error");
  const text = result.content.find(({ type }) => type === "text");
  const reply = JSON.parse(text.text);
  assert.equal(reply.status, "complete");
  assert.equal(reply.result.items[0].card.nct_id, record);
  if (modernSdk) {
    assert.deepEqual(result.structuredContent, reply);
  }
  if (clientPackage === "@modelcontextprotocol/client" && mode === "auto") {
    step = "skills/list";
    const { SkillsClient } = await import("@modelcontextprotocol/client/ext/skills");
    const skills = new SkillsClient(client);
    const { skills: entries } = await skills.list();
    const research = entries.find(({ frontmatter }) => frontmatter.name === "biomcp-research");
    assert(research, "biomcp-research skill missing");
    step = "read SKILL.md";
    assert.equal((await skills.read(research)).verified, true);
    const reference = "references/09-trial-recruitment.md";
    step = `read ${reference}`;
    assert.equal((await skills.read(research, reference)).verified, true);
    console.log(`TypeScript SDK verified biomcp-research SKILL.md and ${reference}`);
  }
} catch (error) {
  console.error(`${step} failed`);
  throw error;
} finally {
  await client.close();
}
