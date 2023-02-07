import { Navbar, NavLink } from "@mantine/core";

export default function MyNavbar() {
    return (
        <Navbar width={{ base: 200 }} p="xs">
            <NavLink label="Replay Positions" active />
        </Navbar>
    );
}
