"use client";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Users, Vote, Clock, TrendingUp } from "lucide-react";

type Proposal = {
  id: string;
  title: string;
  status: "active" | "passed" | "pending";
  votes: number;
  timeLeft: string;
};

const mockProposals: Proposal[] = [
  {
    id: "BP-001",
    title: "Expand Merchant Registry to NCR Zone",
    status: "active",
    votes: 847,
    timeLeft: "2d 14h"
  },
  {
    id: "BP-002", 
    title: "Update PQS Standards v2.1",
    status: "pending",
    votes: 623,
    timeLeft: "5d 8h"
  }
];

export function GovernancePanel() {
  return (
    <Card className="border-primary/20 bg-card/50 backdrop-blur">
      <CardHeader>
        <div className="flex items-center gap-3">
          <div className="rounded-lg bg-primary/10 p-2">
            <Users className="h-5 w-5 text-primary" />
          </div>
          <div>
            <CardTitle className="text-lg">Council of Elders</CardTitle>
            <CardDescription>DAO Governance & Proposals</CardDescription>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="grid grid-cols-3 gap-2">
          <div className="rounded-lg border border-border/50 bg-background/50 p-3">
            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground">Active</span>
              <span className="text-xl font-bold text-primary">12</span>
            </div>
          </div>
          <div className="rounded-lg border border-border/50 bg-background/50 p-3">
            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground">Votes Cast</span>
              <span className="text-xl font-bold text-accent">3.2K</span>
            </div>
          </div>
          <div className="rounded-lg border border-border/50 bg-background/50 p-3">
            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground">Pioneers</span>
              <span className="text-xl font-bold">856</span>
            </div>
          </div>
        </div>

        <div className="space-y-2">
          <h4 className="text-sm font-semibold">Recent Proposals</h4>
          {mockProposals.map((proposal) => (
            <div
              key={proposal.id}
              className="space-y-2 rounded-lg border border-border/50 bg-background/50 p-3"
            >
              <div className="flex items-start justify-between gap-2">
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <Badge variant="outline" className="font-mono text-xs">
                      {proposal.id}
                    </Badge>
                    <Badge
                      variant={proposal.status === "active" ? "default" : "secondary"}
                      className="text-xs"
                    >
                      {proposal.status}
                    </Badge>
                  </div>
                  <p className="text-sm font-medium leading-tight">{proposal.title}</p>
                </div>
              </div>
              <div className="flex items-center justify-between text-xs">
                <div className="flex items-center gap-1 text-muted-foreground">
                  <Vote className="h-3 w-3" />
                  <span>{proposal.votes} votes</span>
                </div>
                <div className="flex items-center gap-1 text-muted-foreground">
                  <Clock className="h-3 w-3" />
                  <span>{proposal.timeLeft}</span>
                </div>
              </div>
            </div>
          ))}
        </div>

        <Button variant="outline" className="w-full" size="sm">
          <TrendingUp className="mr-2 h-4 w-4" />
          View All Proposals
        </Button>
      </CardContent>
    </Card>
  );
}
