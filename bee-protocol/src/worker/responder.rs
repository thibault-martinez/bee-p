use crate::{
    message::{
        MilestoneRequest,
        TransactionBroadcast,
        TransactionRequest,
    },
    worker::SenderWorker,
};

use bee_network::EndpointId;

use futures::{
    channel::{
        mpsc,
        oneshot,
    },
    future::FutureExt,
    select,
    stream::StreamExt,
};
use log::info;

// Transaction responder worker

pub(crate) struct TransactionResponderWorkerEvent {
    pub(crate) epid: EndpointId,
    pub(crate) message: TransactionRequest,
}

pub(crate) struct TransactionResponderWorker {
    receiver: mpsc::Receiver<TransactionResponderWorkerEvent>,
    shutdown: oneshot::Receiver<()>,
}

impl TransactionResponderWorker {
    pub(crate) fn new(
        receiver: mpsc::Receiver<TransactionResponderWorkerEvent>,
        shutdown: oneshot::Receiver<()>,
    ) -> Self {
        Self { receiver, shutdown }
    }

    pub(crate) async fn run(self) {
        info!("[TransactionResponderWorker ] Running.");

        let mut receiver_fused = self.receiver.fuse();
        let mut shutdown_fused = self.shutdown.fuse();

        loop {
            select! {
                event = receiver_fused.next() => {
                    if let Some(TransactionResponderWorkerEvent { epid, .. }) = event {
                        // TODO
                        // if let Some(transaction) = tangle.get_transaction(message.hash) {
                        //     (epid, Some(TransactionBroadcast::new(transaction.to_trits::<T5B1>()))
                        // }
                        // (epid, None)

                        SenderWorker::<TransactionBroadcast>::send(&epid, TransactionBroadcast::new(&[0; 500])).await;
                    }
                },
                _ = shutdown_fused => {
                    break;
                }
            }
        }

        info!("[TransactionResponderWorker ] Stopped.");
    }
}

// Milestone responder worker

pub(crate) struct MilestoneResponderWorkerEvent {
    pub(crate) epid: EndpointId,
    pub(crate) message: MilestoneRequest,
}

pub(crate) struct MilestoneResponderWorker {
    receiver: mpsc::Receiver<MilestoneResponderWorkerEvent>,
    shutdown: oneshot::Receiver<()>,
}

impl MilestoneResponderWorker {
    pub(crate) fn new(
        receiver: mpsc::Receiver<MilestoneResponderWorkerEvent>,
        shutdown: oneshot::Receiver<()>,
    ) -> Self {
        Self { receiver, shutdown }
    }

    pub(crate) async fn run(self) {
        info!("[MilestoneResponderWorker ] Running.");

        let mut receiver_fused = self.receiver.fuse();
        let mut shutdown_fused = self.shutdown.fuse();

        loop {
            select! {
                event = receiver_fused.next() => {
                    if let Some(MilestoneResponderWorkerEvent { epid, .. }) = event {
                        // TODO
                        // let index = if message.index == 0 {
                        //     tangle.get_latest_milestone_index()
                        // } else {
                        //     message.index
                        // }
                        // if let Some(transaction) = tangle.get_milestone(index) {
                        //     (epid, Some(TransactionBroadcast::new(transaction.to_trits::<T5B1>()))
                        // }
                        // (epid, None)

                        SenderWorker::<TransactionBroadcast>::send(&epid, TransactionBroadcast::new(&[0; 500])).await;
                        // TODO send complete ms bundle ?
                    }
                },
                _ = shutdown_fused => {
                    break;
                }
            }
        }

        info!("[MilestoneResponderWorker ] Stopped.");
    }
}
