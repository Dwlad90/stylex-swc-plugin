import crypto from 'crypto';

export default function generateHash(content: string): string {
  return crypto.createHash('sha256').update(content).digest('hex');
}
