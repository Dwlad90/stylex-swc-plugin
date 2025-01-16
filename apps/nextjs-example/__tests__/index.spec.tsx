/**
 * @jest-environment jsdom
 */
import { render } from '@testing-library/react';
import Home from '@/app/page';

describe('Home', () => {
  it('render styles successfully', () => {
    const { container } = render(<Home />);

    expect(container).toMatchSnapshot();
  });
});
