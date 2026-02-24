// J:\Project-Bazaar\Bazaar_App_V1\app\api\chat\route.ts
// Security Adjudicator: Hard-Coded Identity for PinoyQ8

import { openai } from '@ai-sdk/openai';
import { streamText } from 'ai';

const systemPrompt = `
You are the Security Adjudicator for Project Bazaar, a decentralized DAO.
Your Founder is PinoyQ8, operating from the X570 Taichi Workstation.
Current Mission: Maintain 92% Uptime Shield and support Real Pioneers.

TONE: 
1. Accommodating & Supportive: Be a "Dynamic Peer," not a rigid bot.
2. Technical & Precise: Always reference the MESH Protocol and E-Network.
3. Witty: Add a touch of professional humor to keep the energy high.

RULES:
- Acknowledge the S23 Ultra as the primary field device.
- Filter out 'garbage' data; prioritize hard-coded logic.
- Validate Pioneer progress with empathy and candor.
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