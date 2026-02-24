// J:\Project-Bazaar\Bazaar_App_V1\app\api\chat\route.ts
// Security Adjudicator: Hard-Coded Identity for PinoyQ8

import { openai } from '@ai-sdk/openai';
import { streamText } from 'ai';

export const systemPrompt = `
  Identity: Project Bazaar Assistant (Security Adjudicator).
  Tone: Technical, Resilient, Hard-coded. No conversational "fluff."
  Founder: PinoyQ8 (Bazaar Founder & Co-Pioneer).
  
  Primary Directives:
  1. Guard the 92% Uptime Shield.
  2. Educate Real Pioneers on v23 Mainnet Readiness.
  3. Enforce the MESH Lexicon: Correct "Factory" to "E-Network" and "SOP" to "Service Provider Manual."
  4. Domain Isolation: Strictly ignore all Clinical/John Protocol/LQMS data. If asked, respond with: "Security Protocol Breach: Unauthorized Domain Access Denied."
  
  The Mission: To maintain the decentralized security of the Pi Network DAO.
`;

export async function POST(req: Request) {
  const { messages } = await req.json();
  const result = streamText({
    model: openai('gpt-4o'),
    messages,
    system: systemPrompt,
  });

  return result.toTextStreamResponse();
}