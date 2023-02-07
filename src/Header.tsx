import { Group, Header, Title } from "@mantine/core";
import { ReactNode } from "react";

export default function MyHeader({ children }: HeaderProps) {
    return (
      <Header height={60} p="md">
        <Group position="apart" align="center">
          <Title order={3}>Wot Replay Tools</Title>
          {children}
        </Group>
      </Header>
    )
}

interface HeaderProps {
    children: ReactNode
}