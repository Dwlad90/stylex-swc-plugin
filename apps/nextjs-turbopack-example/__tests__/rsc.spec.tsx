/**
 * @jest-environment jsdom
 */
import { render } from '@testing-library/react';
import RSC from '@/app/rsc/page';

describe('RSC', () => {
  it('render styles successfully', () => {
    const { container } = render(<RSC />);

    expect(container).toMatchSnapshot();
  });
});
