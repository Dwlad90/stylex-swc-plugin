import { describe, it, expect } from 'vitest';
import { composeStories } from '@storybook/react';
import { render } from '@testing-library/react';
import * as CardStories from './Card.stories';


describe('Card Snapshots', () => {
  const stories = composeStories(CardStories);

  Object.entries(stories).forEach(([name, Story]) => {
    it(`renders ${name} story correctly`, () => {
      const { container } = render(<Story />);
      expect(container.firstChild).toMatchSnapshot();
    });
  });
});

