pragma solidity ^0.8.0;

error VotingPeriodEnded(uint256 voteSubmissionDeadline, uint256 request_time);
error InvalidVote(uint256 voteValue);
error NullifierAlreadyUsed(uint256 nullifier);
error RevotingNotAllowed();
error InvalidMerkleRoot(uint256 provided, uint256 expected);
error InvalidStateroot(uint256 provided, uint256 expected);
error InvalidCeremonyId(uint256 provided, uint256 expected);
error InvalidProof();
error InvalidRevoteValue();
error NullifierMismatch(uint256 voteNullifier, uint256 revoteNullifier);
error InvalidCeremonyType();
error DeploymentFailed();
error SaltAlreadyUsed(bytes32 salt);

