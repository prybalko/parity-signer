package io.parity.signer.modals

import android.widget.ImageView
import android.widget.Toast
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject
import kotlin.math.roundToInt
import kotlin.math.sign

@ExperimentalMaterialApi
@ExperimentalAnimationApi
@Composable
fun ExportPublicKey(signerDataModel: SignerDataModel) {
	val swipeableState = rememberSwipeableState(initialValue = 0)

	Column(
		modifier = Modifier
			.fillMaxWidth()
			.swipeable(
				orientation = Orientation.Horizontal,
				state = swipeableState,
				anchors = mapOf(0f to 0, 400.0f to 1, -400.0f to -1),
				thresholds = { _, _ -> FractionalThreshold(0.8f) }
			)
	) {
		Column(Modifier.padding(8.dp)) {

			Spacer(Modifier.padding(8.dp))
			//Text(selectedNetwork.value!!.get("title").toString())
		}
	}
}
